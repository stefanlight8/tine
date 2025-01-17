use std::io::{Error, ErrorKind};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::from_utf8;

pub struct ClientId {
    pub addr: SocketAddr,
}

impl ClientId {
    /// Get id string, not an IP address.
    pub fn to_string(&self) -> String {
        match self.addr {
            SocketAddr::V4(addr) => {
                let ip = addr.ip();
                let port = addr.port();
                let ip_str = format!(
                    "{:02x}{:02x}{:02x}{:02x}",
                    ip.octets()[0],
                    ip.octets()[1],
                    ip.octets()[2],
                    ip.octets()[3]
                );
                let port_str = format!("{:04x}", port);
                format!("{}{}", ip_str, port_str)
            }
            _ => self.addr.to_string(),
        }
    }

    /// Get id from IP address in string.
    pub fn from_string(addr: &str) -> Result<Self, Error> {
        let bytes: Result<Vec<u8>, _> = addr
            .as_bytes()
            .chunks(2)
            .map(|chunk| u8::from_str_radix(from_utf8(chunk).unwrap_or("0"), 16))
            .collect();

        let bytes =
            bytes.map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid address format"))?;

        if bytes.len() != 6 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid address format",
            ));
        }

        let ip = Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]);
        let port = u16::from_be_bytes([bytes[4], bytes[5]]);
        let socket_addr = SocketAddr::new(IpAddr::V4(ip), port);
        Ok(ClientId { addr: socket_addr })
    }
}
