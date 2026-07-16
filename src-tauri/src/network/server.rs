use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::state::AppState;
use crate::network::protocol::{MetadataMessage, AckMessage, ErrorMessage, ProgressEvent};
use std::path::PathBuf;
use tokio::fs::File;
use std::time::Instant;

pub async fn run_server(
    app_handle: AppHandle,
    state: Arc<Mutex<AppState>>,
    start_port: u16,
) -> Result<(), String> {
    let mut port = start_port;
    let listener = loop {
        match TcpListener::bind(format!("0.0.0.0:{}", port)).await {
            Ok(l) => break l,
            Err(_) => {
                port += 1;
                if port > start_port + 100 {
                    return Err("Could not find open port".into());
                }
            }
        }
    };

    {
        let mut st = state.lock().await;
        st.server_port = Some(port);
    }

    app_handle.emit("server-started", port).map_err(|e| e.to_string())?;

    loop {
        let (stream, addr) = listener.accept().await.map_err(|e| e.to_string())?;
        let app = app_handle.clone();
        let peer_addr = addr.ip().to_string();
        
        let (rx_socket, mut tx_socket) = stream.into_split();
        let (tx, mut rx) = mpsc::channel::<String>(32);

        {
            let mut st = state.lock().await;
            st.active_clients.insert(peer_addr.clone(), tx.clone());
        }

        let app_clone = app.clone();
        let peer_clone = peer_addr.clone();
        
        let tx_clone = tx.clone(); // Clone channel to send messages from read loop

        tokio::spawn(async move {
            let mut reader = BufReader::new(rx_socket);
            let mut first_line = String::new();
            
            if let Ok(bytes_read) = reader.read_line(&mut first_line).await {
                if bytes_read == 0 { return; }

                if let Ok(meta) = serde_json::from_str::<MetadataMessage>(&first_line) {
                    if meta.r#type == "metadata" {
                        let download_dir = dirs::download_dir().unwrap_or_else(|| PathBuf::from("."));
                        let save_path = download_dir.join(&meta.file_name);
                        
                        match File::create(&save_path).await {
                            Ok(mut file) => {
                                let mut total_received = 0u64;
                                let start_time = Instant::now();
                                let mut last_progress = Instant::now();
                                
                                loop {
                                    if total_received >= meta.file_size {
                                        break; 
                                    }
                                    
                                    let chunk_index = match reader.read_u64().await {
                                        Ok(v) => v,
                                        Err(_) => break,
                                    };
                                    let chunk_len = match reader.read_u32().await {
                                        Ok(v) => v,
                                        Err(_) => break,
                                    };
                                    let chunk_crc = match reader.read_u32().await {
                                        Ok(v) => v,
                                        Err(_) => break,
                                    };
                                    
                                    let mut chunk_buf = vec![0; chunk_len as usize];
                                    if let Err(_) = reader.read_exact(&mut chunk_buf).await {
                                        break;
                                    }
                                    
                                    let crc = crc32fast::hash(&chunk_buf);
                                    if crc != chunk_crc {
                                        let err_msg = ErrorMessage {
                                            r#type: "error".to_string(),
                                            transfer_id: meta.transfer_id.clone(),
                                            code: "CHECKSUM_MISMATCH".to_string(),
                                            message: format!("Chunk {} corrupted", chunk_index),
                                        };
                                        let err_json = serde_json::to_string(&err_msg).unwrap();
                                        let _ = tx_clone.send(format!("{}\n", err_json)).await;
                                        return;
                                    }
                                    
                                    if let Err(e) = file.write_all(&chunk_buf).await {
                                        let _ = app_clone.emit("network-message", (peer_clone.clone(), format!("File write error: {}", e)));
                                        return;
                                    }
                                    
                                    total_received += chunk_len as u64;
                                    
                                    let ack = AckMessage {
                                        r#type: "ack".to_string(),
                                        transfer_id: meta.transfer_id.clone(),
                                        chunk_index,
                                    };
                                    let ack_json = serde_json::to_string(&ack).unwrap();
                                    let _ = tx_clone.send(format!("{}\n", ack_json)).await;
                                    
                                    if last_progress.elapsed().as_millis() > 100 {
                                        let elapsed = start_time.elapsed().as_secs_f64();
                                        let speed = if elapsed > 0.0 { (total_received as f64 / 1_048_576.0) / elapsed } else { 0.0 };
                                        let _ = app_clone.emit("transfer-progress", ProgressEvent {
                                            transfer_id: meta.transfer_id.clone(),
                                            file_name: meta.file_name.clone(),
                                            sent_bytes: total_received,
                                            total_bytes: meta.file_size,
                                            percentage: (total_received as f64 / meta.file_size as f64) * 100.0,
                                            speed_mbps: speed,
                                            is_done: false,
                                        });
                                        last_progress = Instant::now();
                                    }
                                }
                                
                                if total_received == meta.file_size {
                                    let elapsed = start_time.elapsed().as_secs_f64();
                                    let speed = if elapsed > 0.0 { (total_received as f64 / 1_048_576.0) / elapsed } else { 0.0 };
                                    
                                    let _ = app_clone.emit("transfer-progress", ProgressEvent {
                                        transfer_id: meta.transfer_id.clone(),
                                        file_name: meta.file_name.clone(),
                                        sent_bytes: total_received,
                                        total_bytes: meta.file_size,
                                        percentage: 100.0,
                                        speed_mbps: speed,
                                        is_done: true,
                                    });
                                    let _ = app_clone.emit("network-message", (peer_clone.clone(), format!("File received successfully: {:?}", save_path)));
                                } else {
                                    let _ = app_clone.emit("network-message", (peer_clone.clone(), format!("File size mismatch.")));
                                }
                            }
                            Err(e) => {
                                let _ = app_clone.emit("network-message", (peer_clone.clone(), format!("Failed to create file: {}", e)));
                            }
                        }
                        return;
                    }
                }
                
                let _ = app_clone.emit("network-message", (peer_clone.clone(), first_line));
                let mut buf = [0; 4096];
                loop {
                    match reader.read(&mut buf).await {
                        Ok(0) => break,
                        Ok(n) => {
                            let msg = String::from_utf8_lossy(&buf[..n]).to_string();
                            let _ = app_clone.emit("network-message", (peer_clone.clone(), msg));
                        }
                        Err(_) => break,
                    }
                }
            }
        });

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if tx_socket.write_all(msg.as_bytes()).await.is_err() {
                    break;
                }
            }
        });
    }
}
