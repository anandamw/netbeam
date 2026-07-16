use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferRecord {
    pub file_name: String,
    pub file_size: u64,
    pub timestamp: String,
    pub peer: String,
    pub direction: String, // "sent" or "received"
    pub status: String, // "success" or "failed"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecentDevice {
    pub ip: String,
    pub port: u16,
    pub last_seen: String,
}

fn get_data_dir() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("Netbeam");
    if !path.exists() {
        let _ = fs::create_dir_all(&path);
    }
    path
}

pub fn get_history() -> Vec<TransferRecord> {
    let path = get_data_dir().join("history.json");
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(history) = serde_json::from_str(&content) {
            return history;
        }
    }
    Vec::new()
}

pub fn add_history_record(record: TransferRecord) {
    let mut history = get_history();
    history.insert(0, record); // add to top
    if history.len() > 100 {
        history.truncate(100);
    }
    
    let path = get_data_dir().join("history.json");
    let _ = fs::write(path, serde_json::to_string_pretty(&history).unwrap_or_default());
}

pub fn get_recent_devices() -> Vec<RecentDevice> {
    let path = get_data_dir().join("devices.json");
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(devices) = serde_json::from_str(&content) {
            return devices;
        }
    }
    Vec::new()
}

pub fn add_recent_device(device: RecentDevice) {
    let mut devices = get_recent_devices();
    devices.retain(|d| d.ip != device.ip); // remove old entry if exists
    devices.insert(0, device); // add to top
    if devices.len() > 20 {
        devices.truncate(20);
    }
    
    let path = get_data_dir().join("devices.json");
    let _ = fs::write(path, serde_json::to_string_pretty(&devices).unwrap_or_default());
}
