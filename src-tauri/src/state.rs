use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub struct AppState {
    pub server_port: Option<u16>,
    pub active_clients: HashMap<String, mpsc::Sender<String>>,
    pub pending_transfers: HashMap<String, oneshot::Sender<bool>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            server_port: None,
            active_clients: HashMap::new(),
            pending_transfers: HashMap::new(),
        }
    }
}
