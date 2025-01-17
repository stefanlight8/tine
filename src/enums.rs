use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum RequestType {
    Handshake,
    FileRequest(bool),
    FileAvailable {
        file_id: u64,
        file_name: String,
        file_size: u64,
    },
    FileChunk {
        file_id: u64,
        chunk_index: u32,
        data: Vec<u8>,
    },
    Ack {
        file_id: u64,
        chunk_index: u32,
    },
}
