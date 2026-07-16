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
) -> Result<String, String> {
    tokio::spawn(async move {
        let _ = sender::send_file(app, ip, port, file_path).await;
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
