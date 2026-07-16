use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub struct AppState {
    pub server_port: Option<u16>,
    pub server_abort: Option<oneshot::Sender<()>>,
    pub active_clients: HashMap<String, mpsc::Sender<String>>,
    pub pending_transfers: HashMap<String, oneshot::Sender<bool>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            server_port: None,
            server_abort: None,
            active_clients: HashMap::new(),
            pending_transfers: HashMap::new(),
        }
    }
}
