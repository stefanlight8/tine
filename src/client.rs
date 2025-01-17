use std::{io::Error, net::SocketAddr, path::Path};

use serde::{de::DeserializeOwned, Serialize};
use tokio::{fs, net::UdpSocket};

use crate::{
    enums::RequestType,
    utils::{deserialize, get_file_id, serialize},
};

pub struct Client {
    socket: Option<UdpSocket>,
}

impl Client {
    pub fn new() -> Self {
        Client { socket: None }
    }

    /// Bind UPD socket to client
    pub async fn bind(&mut self, addr: SocketAddr) -> Result<(), Error> {
        let socket = UdpSocket::bind(addr).await?;
        self.socket = Some(socket);
        Ok(())
    }

    /// Connect to peer
    pub async fn connect(&self, addr: SocketAddr) -> Result<(), Error> {
        self.send(&RequestType::Handshake, addr).await?;
        self.recv::<RequestType>().await?;
        Ok(())
    }

    /// Send socket
    pub async fn send<T>(&self, data: &T, addr: SocketAddr) -> Result<(), Error>
    where
        T: Serialize,
    {
        let data = serialize(data).expect("Failed to serialize");
        self.socket
            .as_ref()
            .expect("Socket is not binded") // TODO: other format
            .send_to(&data, addr)
            .await?;
        Ok(())
    }

    /// Get data from socket
    pub async fn recv<T>(&self) -> Result<(T, SocketAddr), Error>
    where
        T: DeserializeOwned,
    {
        let mut buf = vec![0u8; 1400];
        let (len, addr) = self
            .socket
            .as_ref()
            .expect("Socket is not binded") // TODO: other format
            .recv_from(&mut buf)
            .await?;
        buf.truncate(len);
        let data = deserialize(&buf).expect("Failed to serialize");
        Ok((data, addr))
    }

    pub async fn send_file_data(&self, addr: SocketAddr, file_path: &str) -> Result<(), Error> {
        let file_name = Path::new(file_path)
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let file_size = match fs::metadata(file_path).await {
            Ok(metadata) => metadata.len(),
            Err(_) => 0,
        };
        self.send(
            &RequestType::FileAvailable {
                file_id: get_file_id(file_name, file_size),
                file_name: file_name.to_string(),
                file_size,
            },
            addr,
        )
        .await
    }
}
