use std::env;
use std::io::{Error, ErrorKind};
use std::net::ToSocketAddrs;

use tokio;

use client::Client;
use enums::RequestType;
use structs::ClientId;
use utils::{get_external_address, request_peer_client_id};

mod client;
mod enums;
mod structs;
mod utils;

const DEFAULT_STUN_HOST: &'static str = "108.177.119.127"; // Google stun host

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let stun_host = env::var("STUN_HOST").unwrap_or(DEFAULT_STUN_HOST.to_string());
    println!("using {} as stun host", stun_host);

    let client_id = match get_external_address(stun_host).await {
        Some(addr) => ClientId { addr: addr },
        None => {
            return Err(Error::new(
                ErrorKind::AddrNotAvailable,
                "Cannot get external address",
            ))
        }
    };
    let local_addr = format!("0.0.0.0:{}", client_id.addr.port())
        .to_socket_addrs()?
        .next()
        .expect("No socket address");

    let mut client = Client::new();
    client.bind(local_addr).await?;

    println!("🐙 Hello, your client id is {}", &client_id.to_string());
    let peer_id = request_peer_client_id()?;

    match args[1].as_str() {
        "send" => {
            if client.connect(peer_id.addr).await.is_ok() {
                println!("connected");
            }
            let file_path = &args[2];
            client.send_file_data(peer_id.addr, file_path).await?;
            let (data, _) = client.recv::<RequestType>().await?;
            match data {
                RequestType::FileRequest(true) => todo!("do file response"),
                _ => {}
            }
        }
        "get" => todo!("do get mode"),
        &_ => return Err(Error::new(ErrorKind::InvalidInput, "Unknown mode")),
    }

    Ok(())
}
