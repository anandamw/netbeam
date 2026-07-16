use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::net::UdpSocket;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscoveryMessage {
    pub device_name: String,
    pub ip: String,
    pub port: u16,
}

pub async fn run_discovery(app: AppHandle, device_name: String, tcp_port: u16) -> Result<(), String> {
    let udp_port = 3001;
    
    // Bind the socket
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", udp_port))
        .await
        .map_err(|e| format!("Failed to bind UDP socket: {}", e))?;
    
    socket.set_broadcast(true).map_err(|e| format!("Failed to set broadcast: {}", e))?;
    let socket = Arc::new(socket);
    
    let announcer_socket = socket.clone();
    let name_clone = device_name.clone();
    
    // Spawn Announcer
    tokio::spawn(async move {
        loop {
            // Find local IP (stub: for now we use 127.0.0.1 if we can't find real IP, 
            // but for a real app we should enumerate interfaces.
            // Using local_addr is not enough, but for broadcast, the receiver gets the sender IP automatically).
            let msg = DiscoveryMessage {
                device_name: name_clone.clone(),
                ip: "auto".to_string(), // receiver will use addr.ip()
                port: tcp_port,
            };
            
            if let Ok(json) = serde_json::to_string(&msg) {
                // Broadcast to 255.255.255.255
                let _ = announcer_socket.send_to(json.as_bytes(), format!("255.255.255.255:{}", udp_port)).await;
            }
            
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });
    
    // Listener loop
    let mut buf = [0; 1024];
    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                let data = &buf[..len];
                if let Ok(mut msg) = serde_json::from_slice::<DiscoveryMessage>(data) {
                    // Inject the actual IP
                    msg.ip = addr.ip().to_string();
                    let _ = app.emit("device-discovered", msg);
                }
            }
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
