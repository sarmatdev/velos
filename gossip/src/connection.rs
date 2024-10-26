use std::{
    io::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UdpSocket,
};

use udpflow::UdpStreamRemote;

pub struct Connection {
    socket: UdpStreamRemote,
}

impl Connection {
    pub async fn connect(addr: SocketAddr) -> Result<Connection, Error> {
        let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
        let local_socket = UdpSocket::bind(local_addr).await?;
        let socket = UdpStreamRemote::new(local_socket, addr);
        Ok(Self { socket })
    }

    pub async fn receive(&mut self) -> Result<Option<[u8; 2000]>, Error> {
        let mut buf = [0; 2000];

        let len = self.socket.read(&mut buf).await?;

        if len > 0 {
            Ok(Some(buf))
        } else {
            Ok(None)
        }
    }

    pub async fn send(&mut self, packet_serealized: Vec<u8>) -> Result<(), Error> {
        self.socket.write_all(&packet_serealized).await?;

        Ok(())
    }

    #[allow(unused)]
    pub fn entrypoint_addr(&self) -> SocketAddr {
        self.socket.peer_addr()
    }
    #[allow(unused)]
    pub fn local_addr(&self) -> SocketAddr {
        self.socket.local_addr()
    }
}
