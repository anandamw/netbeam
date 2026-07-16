use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataMessage {
    pub r#type: String, // Always "metadata"
    pub transfer_id: String,
    pub file_name: String,
    pub file_size: u64,
    pub mime: String,
    pub checksum_sha256: String,
    pub chunk_size: u32,
    pub relative_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AckMessage {
    pub r#type: String, // "ack"
    pub transfer_id: String,
    pub chunk_index: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub r#type: String, // "error"
    pub transfer_id: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgressEvent {
    pub transfer_id: String,
    pub file_name: String,
    pub sent_bytes: u64,
    pub total_bytes: u64,
    pub percentage: f64,
    pub speed_mbps: f64, // MB/s
    pub is_done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairingRequest {
    pub r#type: String, // "pairing_req"
    pub device_name: String,
    pub pin: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlMessage {
    pub r#type: String, // "control"
    pub action: String, // "accept", "reject", "pause", "resume"
    pub transfer_id: Option<String>,
}
