use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct AppState {
    pub server_port: Option<u16>,
    pub active_clients: HashMap<String, mpsc::Sender<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            server_port: None,
            active_clients: HashMap::new(),
        }
    }
}
