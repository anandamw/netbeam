use tauri::{AppHandle, State, command};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::state::AppState;
use crate::network;
use crate::file::sender;

#[command]
pub async fn start_server(
    app: AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
    port: u16,
) -> Result<String, String> {
    let state_arc = state.inner().clone();
    
    tokio::spawn(async move {
        let _ = network::server::run_server(app, state_arc, port).await;
    });
    
    Ok("Server starting...".to_string())
}

#[command]
pub async fn connect_client(
    app: AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
    ip: String,
    port: u16,
) -> Result<String, String> {
    let state_arc = state.inner().clone();
    
    match network::client::connect_to(app, state_arc, ip.clone(), port).await {
        Ok(_) => Ok(format!("Connected to {}:{}", ip, port)),
        Err(e) => Err(e),
    }
}

#[command]
pub async fn send_message(
    state: State<'_, Arc<Mutex<AppState>>>,
    ip: String,
    msg: String,
) -> Result<(), String> {
    let st = state.lock().await;
    if let Some(tx) = st.active_clients.get(&ip) {
        tx.send(msg).await.map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Client not connected".to_string())
    }
}

#[command]
pub async fn send_file(
    app: AppHandle,
    ip: String,
    port: u16,
    file_path: String,
    relative_path: Option<String>,
) -> Result<String, String> {
    tokio::spawn(async move {
        let _ = sender::send_file(app, ip, port, file_path, relative_path).await;
    });
    Ok("File transfer started".into())
}

#[command]
pub async fn start_discovery(
    app: AppHandle,
    device_name: String,
    tcp_port: u16,
) -> Result<String, String> {
    tokio::spawn(async move {
        let _ = network::discovery::run_discovery(app, device_name, tcp_port).await;
    });
    Ok("Discovery started".into())
}

#[command]
pub async fn accept_transfer(
    state: State<'_, Arc<Mutex<AppState>>>,
    transfer_id: String,
) -> Result<(), String> {
    let mut st = state.lock().await;
    if let Some(tx) = st.pending_transfers.remove(&transfer_id) {
        let _ = tx.send(true);
    }
    Ok(())
}

#[command]
pub async fn reject_transfer(
    state: State<'_, Arc<Mutex<AppState>>>,
    transfer_id: String,
) -> Result<(), String> {
    let mut st = state.lock().await;
    if let Some(tx) = st.pending_transfers.remove(&transfer_id) {
        let _ = tx.send(false);
    }
    Ok(())
}

#[command]
pub fn get_history() -> Vec<crate::store::TransferRecord> {
    crate::store::get_history()
}

#[command]
pub fn add_history_record(record: crate::store::TransferRecord) {
    crate::store::add_history_record(record);
}

#[command]
pub fn get_recent_devices() -> Vec<crate::store::RecentDevice> {
    crate::store::get_recent_devices()
}

#[command]
pub fn add_recent_device(device: crate::store::RecentDevice) {
    crate::store::add_recent_device(device);
}

#[derive(serde::Serialize)]
pub struct ExpandedPath {
    pub absolute_path: String,
    pub relative_path: String,
}

#[command]
pub fn expand_paths(paths: Vec<String>) -> Result<Vec<ExpandedPath>, String> {
    let mut expanded = Vec::new();
    
    for path_str in paths {
        let path = std::path::Path::new(&path_str);
        if !path.exists() {
            continue;
        }
        
        if path.is_file() {
            let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            expanded.push(ExpandedPath {
                absolute_path: path_str.clone(),
                relative_path: file_name,
            });
        } else if path.is_dir() {
            let folder_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            for entry in walkdir::WalkDir::new(&path) {
                if let Ok(entry) = entry {
                    if entry.path().is_file() {
                        if let Ok(rel_path) = entry.path().strip_prefix(&path) {
                            let rel_str = format!("{}/{}", folder_name, rel_path.to_string_lossy().replace("\\", "/"));
                            expanded.push(ExpandedPath {
                                absolute_path: entry.path().to_string_lossy().to_string(),
                                relative_path: rel_str,
                            });
                        }
                    }
                }
            }
        }
    }
    
    Ok(expanded)
}

#[command]
pub fn get_trusted_devices() -> Vec<crate::store::TrustedDevice> {
    crate::store::get_trusted_devices()
}

#[command]
pub async fn pair_device(
    ip: String,
    port: u16,
    device_name: String,
    pin: String,
) -> Result<String, String> {
    use tokio::net::TcpStream;
    use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
    
    let pairing_req = crate::network::protocol::PairingRequest {
        r#type: "pairing_req".to_string(),
        device_name,
        pin,
    };
    
    let req_json = serde_json::to_string(&pairing_req).map_err(|e| e.to_string())?;
    
    let stream = TcpStream::connect(format!("{}:{}", ip, port))
        .await
        .map_err(|e| e.to_string())?;
        
    let (rx_socket, mut tx_socket) = stream.into_split();
    
    let payload = format!("{}\n", req_json);
    tx_socket.write_all(payload.as_bytes()).await.map_err(|e| e.to_string())?;
    
    let mut reader = BufReader::new(rx_socket);
    let mut control_line = String::new();
    match reader.read_line(&mut control_line).await {
        Ok(0) => return Err("Connection closed before confirmation".into()),
        Ok(_) => {
            if let Ok(ctrl) = serde_json::from_str::<crate::network::protocol::ControlMessage>(&control_line) {
                if ctrl.action == "reject" {
                    return Err("Pairing rejected by receiver".into());
                } else if ctrl.action == "accept" {
                    crate::store::add_trusted_device(crate::store::TrustedDevice {
                        ip: ip.clone(),
                        name: "Paired Device".to_string(),
                    });
                    return Ok("Paired successfully!".to_string());
                } else {
                    return Err("Invalid control response".into());
                }
            } else {
                return Err(format!("Failed to parse ControlMessage: {}", control_line));
            }
        }
        Err(e) => return Err(e.to_string()),
    }
}
