use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use uuid::Uuid;
use crate::network::protocol::{MetadataMessage, AckMessage, ProgressEvent};
use crate::file::checksum;
use std::time::Instant;
use tauri::{AppHandle, Emitter};

pub async fn send_file(app: AppHandle, ip: String, port: u16, file_path: String) -> Result<(), String> {
    let path = Path::new(&file_path);
    let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let metadata_info = std::fs::metadata(&path).map_err(|e| e.to_string())?;
    let file_size = metadata_info.len();
    
    let sha256 = checksum::calculate_sha256(&file_path).map_err(|e| e.to_string())?;
    let transfer_id = Uuid::new_v4().to_string();
    let chunk_size = 65536;
    
    let meta_msg = MetadataMessage {
        r#type: "metadata".to_string(),
        transfer_id: transfer_id.clone(),
        file_name: file_name.clone(),
        file_size,
        mime: "application/octet-stream".to_string(),
        checksum_sha256: sha256,
        chunk_size,
    };
    
    let meta_json = serde_json::to_string(&meta_msg).map_err(|e| e.to_string())?;
    
    let stream = TcpStream::connect(format!("{}:{}", ip, port))
        .await
        .map_err(|e| e.to_string())?;
        
    let (rx_socket, mut tx_socket) = stream.into_split();
    
    // Send metadata JSON followed by a newline separator
    let payload = format!("{}\n", meta_json);
    tx_socket.write_all(payload.as_bytes()).await.map_err(|e| e.to_string())?;
    
    let mut reader = BufReader::new(rx_socket);
    
    // Wait for Confirmation
    let mut control_line = String::new();
    match reader.read_line(&mut control_line).await {
        Ok(0) => return Err("Connection closed before confirmation".into()),
        Ok(_) => {
            if let Ok(ctrl) = serde_json::from_str::<crate::network::protocol::ControlMessage>(&control_line) {
                if ctrl.action == "reject" {
                    return Err("Transfer rejected by receiver".into());
                } else if ctrl.action != "accept" {
                    return Err("Invalid control response".into());
                }
            } else {
                return Err(format!("Failed to parse ControlMessage: {}", control_line));
            }
        }
        Err(e) => return Err(e.to_string()),
    }
    
    let mut file = File::open(&path).map_err(|e| e.to_string())?;
    let mut buffer = vec![0; chunk_size as usize];
    
    let mut chunk_index: u64 = 0;
    let mut sent_bytes = 0u64;
    let start_time = Instant::now();
    let mut last_progress_time = Instant::now();

    loop {
        let count = file.read(&mut buffer).map_err(|e| e.to_string())?;
        if count == 0 {
            break; // EOF
        }
        
        let chunk_data = &buffer[..count];
        let crc = crc32fast::hash(chunk_data);
        
        tx_socket.write_u64(chunk_index).await.map_err(|e| e.to_string())?;
        tx_socket.write_u32(count as u32).await.map_err(|e| e.to_string())?;
        tx_socket.write_u32(crc).await.map_err(|e| e.to_string())?;
        tx_socket.write_all(chunk_data).await.map_err(|e| e.to_string())?;
        
        sent_bytes += count as u64;
        
        let mut ack_line = String::new();
        match reader.read_line(&mut ack_line).await {
            Ok(0) => return Err("Connection closed waiting for ack".into()),
            Ok(_) => {
                if let Ok(ack) = serde_json::from_str::<AckMessage>(&ack_line) {
                    if ack.chunk_index != chunk_index {
                        return Err(format!("Ack mismatch: expected {}, got {}", chunk_index, ack.chunk_index));
                    }
                } else {
                    return Err(format!("Failed to parse Ack: {}", ack_line));
                }
            }
            Err(e) => return Err(e.to_string()),
        }

        chunk_index += 1;
        
        if last_progress_time.elapsed().as_millis() > 100 {
            let elapsed = start_time.elapsed().as_secs_f64();
            let speed = if elapsed > 0.0 { (sent_bytes as f64 / 1_048_576.0) / elapsed } else { 0.0 };
            
            let _ = app.emit("transfer-progress", ProgressEvent {
                transfer_id: transfer_id.clone(),
                file_name: file_name.clone(),
                sent_bytes,
                total_bytes: file_size,
                percentage: (sent_bytes as f64 / file_size as f64) * 100.0,
                speed_mbps: speed,
                is_done: false,
            });
            last_progress_time = Instant::now();
        }
    }
    
    let elapsed = start_time.elapsed().as_secs_f64();
    let speed = if elapsed > 0.0 { (sent_bytes as f64 / 1_048_576.0) / elapsed } else { 0.0 };
    
    let _ = app.emit("transfer-progress", ProgressEvent {
        transfer_id: transfer_id.clone(),
        file_name,
        sent_bytes,
        total_bytes: file_size,
        percentage: 100.0,
        speed_mbps: speed,
        is_done: true,
    });
    
    Ok(())
}
