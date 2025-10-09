use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6},
    sync::Arc,
};

pub struct MulticastService {
    ips: Vec<std::net::IpAddr>,
    sender_ipv4: tokio::net::UdpSocket,
    receiver_ipv4: tokio::net::UdpSocket,
    sender_ipv6: Option<tokio::net::UdpSocket>,
    receiver_ipv6: Option<tokio::net::UdpSocket>,
}

impl MulticastService {
    // ipv4 multicast： 239.255.0.0 - 239.255.255.255
    // ipv6 multicast： FF05::/16
    // const GROUP_IPV4: &'static str = "239.255.0.66:5996";
    // const GROUP_IPV6: &'static str = "FF05::66:5996";
    const MULTICAST_GROUP_PORT: u16 = 5996;

    const MULTICAST_IPV4: Ipv4Addr = Ipv4Addr::new(239, 255, 0, 66);
    const MULTICAST_GROUP_IPV4: SocketAddrV4 = SocketAddrV4::new(Self::MULTICAST_IPV4, Self::MULTICAST_GROUP_PORT);
    const MULTICAST_IPV6: Ipv6Addr = Ipv6Addr::new(0xFF05, 0, 0, 0, 0, 0, 0, 66);
    const MULTICAST_GROUP_IPV6: SocketAddrV6 = SocketAddrV6::new(Self::MULTICAST_IPV6, Self::MULTICAST_GROUP_PORT, 0, 0);

    pub fn new(ips: Vec<std::net::IpAddr>) -> Result<Arc<Self>, anyhow::Error> {
        let has_ipv6 = ips.iter().any(|ip| ip.is_ipv6());
        let receiver_ipv4 = {
            let listen_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, Self::MULTICAST_GROUP_PORT);
            let socket = socket2::Socket::new(socket2::Domain::IPV4, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
            socket.set_reuse_address(true)?;
            #[cfg(not(windows))]
            socket.set_reuse_port(true)?;
            socket.bind(&listen_addr.into())?;
            // let socket : std::net::UdpSocket = socket.into();
            let udp_socket = tokio::net::UdpSocket::from_std(socket.into())?;
            udp_socket.join_multicast_v4(*Self::MULTICAST_GROUP_IPV4.ip(), Ipv4Addr::UNSPECIFIED)?;
            log::info!("Successfully connected to multicast server: {}", Self::MULTICAST_GROUP_IPV4);
            udp_socket
        };

        let sender_ipv4 = {
            let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
            socket.set_multicast_loop_v4(false)?;
            // 设置更大的值（如 32）可以允许消息被路由器转发 (如果路由器配置允许)。
            socket.set_multicast_ttl_v4(32)?;
            let socket = tokio::net::UdpSocket::from_std(socket)?;
            log::info!("[Sender] bind: {:?}", socket.local_addr()?);
            socket
        };

        let receiver_ipv6 = {
            if has_ipv6 {
                let listen_addr = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, Self::MULTICAST_GROUP_PORT, 0, 0);
                let socket = socket2::Socket::new(socket2::Domain::IPV6, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
                socket.set_reuse_address(true)?;
                #[cfg(not(windows))]
                socket.set_reuse_port(true)?;
                socket.bind(&listen_addr.into())?;
                let udp_socket = tokio::net::UdpSocket::from_std(socket.into())?;
                udp_socket.join_multicast_v6(Self::MULTICAST_GROUP_IPV6.ip(), 0)?;
                log::info!("Successfully connected to multicast server: {}", Self::MULTICAST_GROUP_IPV6);
                Some(udp_socket)
            } else {
                None
            }
        };

        let sender_ipv6 = {
            if has_ipv6 {
                let socket = std::net::UdpSocket::bind(":::0")?;
                socket.set_multicast_loop_v6(false)?;
                let socket = tokio::net::UdpSocket::from_std(socket)?;
                log::info!("[Sender] bind: {:?}", socket.local_addr()?);
                Some(socket)
            } else {
                None
            }
        };

        Ok(Arc::new(Self {
            ips,
            sender_ipv4,
            receiver_ipv4,
            sender_ipv6,
            receiver_ipv6,
        }))
    }
    pub fn init(self: Arc<Self>, mut shutdown_receiver: tokio::sync::broadcast::Receiver<()>) -> tokio::task::JoinHandle<Result<(), anyhow::Error>> {
        let rr = tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let mut buf_v6 = [0u8; 1024];
            let has_ipv6 = self.ips.iter().any(|ip| ip.is_ipv6());
            if let Some(receiver_ipv6) = &self.receiver_ipv6 {
                loop {
                    tokio::select! {
                        data_ipv4 = self.receiver_ipv4.recv_from(&mut buf) => {
                            match data_ipv4 {
                                Ok((len, src_addr)) => {
                                    let msg = String::from_utf8_lossy(&buf[..len]);
                                    if msg.trim() == "EXIT" {
                                        println!("[Receiver] 收到退出指令，退出。");
                                        break Err(anyhow::anyhow!("Exit signal sent"));
                                    }
                                }
                                Err(e) => {
                                    log::error!("{}", e);
                                    break Err(anyhow::anyhow!("Error receiving data: {}", e));
                                }
                            }
                        }
                        data_ipv6 = receiver_ipv6.recv_from(&mut buf_v6) => {
                            match data_ipv6 {
                                Ok((len, src_addr)) => {
                                    let msg = String::from_utf8_lossy(&buf_v6[..len]);
                                    if msg.trim() == "EXIT" {
                                        println!("[Receiver] 收到退出指令，退出。");
                                        break Err(anyhow::anyhow!("Exit signal sent"));
                                    }
                                }
                                Err(e) => {
                                    log::error!("{}", e);
                                    break Err(anyhow::anyhow!("Error receiving data: {}", e));
                                }
                            }
                        }
                        _ = shutdown_receiver.recv() => {
                            break Ok(());
                        }
                    }
                }
            } else {
                loop {
                    tokio::select! {
                    data_ipv4 = self.receiver_ipv4.recv_from(&mut buf) => {
                        match data_ipv4 {
                            Ok((len, src_addr)) => {
                                let msg = String::from_utf8_lossy(&buf[..len]);
                                if msg.trim() == "EXIT" {
                                    println!("[Receiver] 收到退出指令，退出。");
                                    break Err(anyhow::anyhow!("Exit signal sent"));
                                }
                            }
                            Err(e) => {
                                log::error!("{}", e);
                                break Err(anyhow::anyhow!("Error receiving data: {}", e));
                            }
                        }
                    }
                    _ = shutdown_receiver.recv() => {
                        break Ok(());
                    }
                    }
                }
            }
        });
        rr
    }

    pub fn uninit(&mut self) -> Result<(), anyhow::Error> {
        self.receiver_ipv4.leave_multicast_v4(Self::MULTICAST_IPV4, Ipv4Addr::UNSPECIFIED)?;
        if let Some(receiver_ipv6) = &self.receiver_ipv6 {
            receiver_ipv6.leave_multicast_v6(&Self::MULTICAST_IPV6, 0)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[tokio::test]
    async fn test_tokio_select() {
        env_logger::builder().is_test(false).filter_level(log::LevelFilter::Debug).init();
        let mut i = 0;
        loop {
            let mut timer2: Option<tokio::time::Sleep> = None;
            let timer1 = tokio::time::sleep(Duration::from_millis(1000));
            tokio::select! {
                _ = timer1 => {
                    log::debug!("hello world");
                    timer2 = Some(tokio::time::sleep(Duration::from_millis(10)));
                    break;
                },
                // Some(_) = conditional_sleeper(timer2) => {
                Some(_) = conditional_sleeper(timer2) => {
                    log::debug!("goodbye cruel world");
                    // break;
                }
            }
            i += 1;
            log::debug!("{}", i);
        }
    }

    async fn conditional_sleeper(t: Option<tokio::time::Sleep>) -> Option<()> {
        log::debug!("called conditional_sleeper");
        match t {
            Some(timer) => Some(timer.await),
            None => None,
        }
    }
}
