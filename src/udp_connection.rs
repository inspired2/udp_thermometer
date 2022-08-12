use std::net::SocketAddr;
use tokio::net::{ToSocketAddrs, UdpSocket};

#[derive(Default, Debug)]
pub struct ThermConnectionBuilder {
    local_addr: Option<SocketAddr>,
    dest_addr: Option<SocketAddr>,
    socket: Option<UdpSocket>,
}
impl ThermConnectionBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn with_local_addr(mut self, addr: impl ToSocketAddrs) -> Self {
        let socket;
        if let Ok(s) = UdpSocket::bind(addr).await {
            socket = s;
        } else {
            socket = UdpSocket::bind("127.0.0.1:0")
                .await
                .expect("Cannot find free local port to bind UdpSocket to");
        }
        self.local_addr = Some(
            socket
                .local_addr()
                .expect("Error getting local adddr for socket conn"),
        );
        self.socket = Some(socket);
        self
    }

    pub async fn connect_to_peer(mut self, addr: impl ToSocketAddrs) -> Self {
        assert!(
            self.socket.is_some(),
            "call with_local_addr first to create UdpSocket"
        );
        let socket = self.socket.take().unwrap();
        socket
            .connect(addr)
            .await
            .expect("Cannot connect to remote socket");
        self.dest_addr = Some(socket.peer_addr().expect("Unable to get peer address"));
        self.socket = Some(socket);
        self
    }

    pub fn build<const N: usize>(self) -> ThermConnection<N> {
        assert!(self.local_addr.is_some());
        assert!(self.socket.is_some());
        ThermConnection {
            socket: self.socket.unwrap(),
        }
    }
}
pub struct ThermConnection<const N: usize> {
    // local_addr: SocketAddr,
    // dest_addr: Option<SocketAddr>,
    socket: UdpSocket,
}
impl<const N: usize> ThermConnection<N> {
    pub async fn send(&mut self, data: [u8; N]) -> Result<(), String> {
        let mut sent_count: usize = 0;
        while sent_count < N {
            let bytes_count = self
                .socket
                .send(&data[sent_count..])
                .await
                .map_err(|e| e.to_string())?;
            println!(
                "sending {bytes_count} bytes: {:?}. Target peer: {:?}",
                data,
                self.socket.peer_addr()
            );
            sent_count += bytes_count;
        }
        debug_assert!(sent_count == N);
        Ok(())
    }
    pub async fn recv_from(
        &mut self,
        buf: &mut [u8; N],
        therm_peer: SocketAddr,
    ) -> Result<(), String> {
        let mut recv_count: usize = 0;
        let mut local_buf = Vec::with_capacity(N);

        while recv_count < N {
            let mut temp_buf = [0; N];
            let (bytes_count, peer) = self
                .socket
                .recv_from(&mut temp_buf)
                .await
                .map_err(|e| e.to_string())?;
            println!(
                "Received {bytes_count} bytes: {:?}. From peer: {:?}",
                &temp_buf, peer
            );
            if peer != therm_peer {
                continue;
            }
            if bytes_count > N - recv_count {
                return Err("Unexpected packet size(too large)".into());
            }

            std::io::Write::write(&mut local_buf, &temp_buf[0..bytes_count]).unwrap();
            recv_count += bytes_count;
        }

        debug_assert!(local_buf.len() == N);
        debug_assert!(recv_count == N);

        if local_buf.len() != N {
            return Err("Invalid data buffer size".into());
        }

        buf.copy_from_slice(&local_buf);

        Ok(())
    }
}
