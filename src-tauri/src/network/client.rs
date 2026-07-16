use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::state::AppState;

pub async fn connect_to(
    app_handle: AppHandle,
    state: Arc<Mutex<AppState>>,
    ip: String,
    port: u16,
) -> Result<(), String> {
    let stream = TcpStream::connect(format!("{}:{}", ip, port))
        .await
        .map_err(|e| e.to_string())?;

    let (mut rx_socket, mut tx_socket) = stream.into_split();
    let (tx, mut rx) = mpsc::channel::<String>(32);

    {
        let mut st = state.lock().await;
        st.active_clients.insert(ip.clone(), tx);
    }

    let app = app_handle.clone();
    let peer_addr = ip.clone();

    tokio::spawn(async move {
        let mut buf = [0; 4096];
        loop {
            match rx_socket.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    let msg = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = app.emit("network-message", (peer_addr.clone(), msg));
                }
                Err(_) => break,
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

    Ok(())
}
