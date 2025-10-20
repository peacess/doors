use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    sync::Arc,
};

use idl::{
    Header, PartnerId, TerminalId, UlidBytes, X25519Public,
    net_discovery_generated::net_discovery::{DnsTerminal, DnsTerminalArgs, Hi, HiArgs, HiFrame, HiFrameArgs},
};
use tokio::runtime::Handle;
use x25519_dalek::PublicKey;

use crate::{
    HeaderType,
    discover::{net::NetHelper, net_discover_type::NetDiscoverType, partner_service_info::PartnerServiceInfo},
    ffi::FfiBytes,
    lib_app::LIB_APP,
};

pub struct MulticastService {
    sender_ipv4: tokio::net::UdpSocket,
    receiver_ipv4: tokio::net::UdpSocket,
    sender_ipv6: Option<tokio::net::UdpSocket>,
    receiver_ipv6: Option<tokio::net::UdpSocket>,
    service_info: PartnerServiceInfo,
    handle: Handle,
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

    pub fn new(handle: Handle) -> Result<Arc<Self>, anyhow::Error> {
        Self::new_with_ips(handle)
    }

    pub fn new_with_ips(handle: Handle) -> Result<Arc<Self>, anyhow::Error> {
        let service_info = PartnerServiceInfo::new();
        let ips = &service_info.net_ips;
        let has_ipv6 = ips
            .iter()
            .any(|ip| ip.ip_v6_link_local.is_unspecified() || ip.ip_v6_unique_local.is_unspecified());
        let receiver_ipv4 = {
            let listen_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, Self::MULTICAST_GROUP_PORT);
            let socket = socket2::Socket::new(socket2::Domain::IPV4, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
            socket.set_reuse_address(true)?;
            #[cfg(not(windows))]
            socket.set_reuse_port(true)?;
            socket.bind(&listen_addr.into())?;
            socket.set_nonblocking(true)?;
            let join_handle = handle.spawn(async move {
                let udp_socket = tokio::net::UdpSocket::from_std(socket.into())?;
                Ok::<tokio::net::UdpSocket, std::io::Error>(udp_socket)
            });
            let udp_socket = handle.block_on(join_handle)??;

            udp_socket.join_multicast_v4(*Self::MULTICAST_GROUP_IPV4.ip(), Ipv4Addr::UNSPECIFIED)?;
            log::info!("Successfully connected to multicast server: {}", Self::MULTICAST_GROUP_IPV4);
            udp_socket
        };

        let sender_ipv4 = {
            let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
            socket.set_multicast_loop_v4(false)?;
            // 设置更大的值（如 32）可以允许消息被路由器转发 (如果路由器配置允许)。
            socket.set_multicast_ttl_v4(32)?;
            socket.set_nonblocking(true)?;
            let join_handle = handle.spawn(async move {
                let socket = tokio::net::UdpSocket::from_std(socket.into())?;
                Ok::<tokio::net::UdpSocket, std::io::Error>(socket)
            });
            let socket = handle.block_on(join_handle)??;
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
                socket.set_nonblocking(true)?;
                let join_handle = handle.spawn(async move {
                    let udp_socket = tokio::net::UdpSocket::from_std(socket.into())?;
                    Ok::<tokio::net::UdpSocket, std::io::Error>(udp_socket)
                });
                let udp_socket = handle.block_on(join_handle)??;
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
                socket.set_nonblocking(true)?;
                let join_handle = handle.spawn(async move {
                    let socket = tokio::net::UdpSocket::from_std(socket.into())?;
                    Ok::<tokio::net::UdpSocket, std::io::Error>(socket)
                });
                let socket = handle.block_on(join_handle)??;
                log::info!("[Sender] bind: {:?}", socket.local_addr()?);
                Some(socket)
            } else {
                None
            }
        };

        Ok(Arc::new(Self {
            sender_ipv4,
            receiver_ipv4,
            sender_ipv6,
            receiver_ipv6,
            service_info,
            handle,
        }))
    }
    pub fn init(self: Arc<Self>, cancel_token: tokio_util::sync::CancellationToken) -> tokio::task::JoinHandle<Result<(), anyhow::Error>> {
        self.handle.clone().spawn(async move {
            {
                // send hi
                log::info!("Sending initial multicast 'hi' message");

                let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
                let buffer = {
                    let hi = {
                        let show_name = builder.create_string("doors_chat");
                        let dns_terminal = self.service_info.to_dns_terminal(&mut builder);
                        Hi::create(
                            &mut builder,
                            &HiArgs {
                                id: Some(&UlidBytes::from(idl::ids::generate_ulid())),
                                dns_terminal: Some(dns_terminal),
                                show_name: Some(show_name),
                            },
                        )
                    };
                    let header = Header::new(
                        hi.value() as u64,
                        HeaderType::NetDiscovery.to_u32(),
                        NetDiscoverType::Hi.to_u32(),
                        &TerminalId::from(self.service_info.terminal_id),
                        &X25519Public::from(&PublicKey::from(&self.service_info.secret)),
                    );
                    let hi_frame = HiFrame::create(
                        &mut builder,
                        &HiFrameArgs {
                            header: Some(&header),
                            hi: Some(hi),
                        },
                    );
                    builder.finish(hi_frame, None);
                    builder.finished_data()
                };

                let msg = "".to_string();

                match self.sender_ipv4.send_to(buffer, &Self::MULTICAST_GROUP_IPV4).await {
                    Ok(sent) => {
                        log::info!("[Sender] Sent {} bytes to {}", sent, Self::MULTICAST_GROUP_IPV4);
                    }
                    Err(e) => {
                        log::error!("[Sender] Error sending to {}: {}", Self::MULTICAST_GROUP_IPV4, e);
                    }
                }
                if let Some(sender_ipv6) = &self.sender_ipv6 {
                    match sender_ipv6.send_to(msg.as_bytes(), &Self::MULTICAST_GROUP_IPV6).await {
                        Ok(sent) => {
                            log::info!("[Sender] Sent {} bytes to {}", sent, Self::MULTICAST_GROUP_IPV6);
                        }
                        Err(e) => {
                            log::error!("[Sender] Error sending to {}: {}", Self::MULTICAST_GROUP_IPV6, e);
                        }
                    }
                }
            }
            let mut buf_v4 = [0u8; 4096];
            let mut buf_v6 = [0u8; 4096];
            let mut err_count = 0;

            loop {
                if cancel_token.is_cancelled() {
                    log::info!("net discovery cancelled");
                    break;
                }
                if err_count > 50 {
                    log::error!("net discovery err count: {}", err_count);
                    break;
                }
                let re = tokio::select! {
                    data_ipv4 = self.receiver_ipv4.recv_from(&mut buf_v4) => {
                        match data_ipv4 {
                            Err(e) => Err(e),
                            Ok((len, addr)) => {
                                self.handle_recv(len,addr,&buf_v4);
                                Ok(())
                            }
                        }
                    }
                    Some(data_ipv6) = async{
                        if let Some(receiver_ipv6) = &self.receiver_ipv6 {
                            Some(receiver_ipv6.recv_from(&mut buf_v6).await)
                        } else {
                            None
                        }
                    } => {
                        match data_ipv6{
                            Err(e) => Err(e),
                            Ok((len, addr)) => {
                                self.handle_recv(len, addr, &buf_v6);
                                Ok(())
                            }
                        }
                    }
                    _ = cancel_token.cancelled() => {
                        Ok(())
                    }
                };
                if let Err(err) = re {
                    err_count += 1;
                    log::error!("Error receiving data: {}", err);
                }
            }
            Ok(())
        })
    }

    fn handle_recv(&self, len: usize, data: SocketAddr, buffer: &[u8]) {
        if let Some(app) = LIB_APP.get() {
            if let Some(call) = app.get_callback() {
                call(FfiBytes::from(buffer[0..len].to_vec()));
            }
        }
    }

    pub fn uninit(&self) -> Result<(), anyhow::Error> {
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
