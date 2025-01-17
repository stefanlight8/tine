use std::error::Error;
use std::io::{self, stdin, stdout, Write as _};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::net::UdpSocket;

use crate::structs::ClientId;

const STUN_REQUEST: [u8; 20] = [
    0x00, 0x01, 0x00, 0x00, 0x21, 0x12, 0xA4, 0x42, 0xD6, 0x85, 0x79, 0xB8, 0x11, 0x03, 0x30, 0x06,
    0x78, 0x69, 0xDF, 0x42,
];

pub async fn get_external_address(stun_host: String) -> Option<SocketAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").await.ok()?;
    let stun_addr: SocketAddr = format!("{}:3478", stun_host).parse().ok()?;

    socket.send_to(&STUN_REQUEST, stun_addr).await.ok()?;

    let mut buf = [0u8; 1024];
    let size = socket.recv(&mut buf).await.ok()?;

    if size >= 28 && buf[0..2] == [0x01, 0x01] {
        let ip = IpAddr::V4(Ipv4Addr::new(buf[28], buf[29], buf[30], buf[31]));
        let port = u16::from_be_bytes([buf[26], buf[27]]);
        Some(SocketAddr::new(ip, port))
    } else {
        None
    }
}

pub fn serialize<T: Serialize>(message: &T) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf = Vec::new();
    message.serialize(&mut Serializer::new(&mut buf))?;
    Ok(buf)
}

pub fn deserialize<T: for<'de> Deserialize<'de>>(data: &[u8]) -> Result<T, Box<dyn Error>> {
    let mut de = Deserializer::new(data);
    let result = T::deserialize(&mut de)?;
    Ok(result)
}

pub fn request_peer_client_id() -> Result<ClientId, io::Error> {
    print!("🌐 Enter peer client id: ");
    stdout().flush().unwrap();
    let mut peer_input = String::new();
    stdin().read_line(&mut peer_input).unwrap();
    ClientId::from_string(peer_input.trim())
}

pub fn get_file_id(file_name: &str, file_size: u64) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(file_name.as_bytes());
    hasher.update(file_size.to_le_bytes());
    let result = hasher.finalize();
    u64::from_le_bytes(result[0..8].try_into().unwrap())
}
