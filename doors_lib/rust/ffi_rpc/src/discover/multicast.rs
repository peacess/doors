use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    sync::Arc,
};

use flatbuffers::bitflags::iter;
use idl::{
    Header, PartnerId, TerminalId, UlidBytes, X25519Public,
    net_discovery_generated::net_discovery::{DnsTerminal, DnsTerminalArgs, Hi, HiArgs, HiFrame, HiFrameArgs},
};
use tokio::runtime::Handle;
use x25519_dalek::PublicKey;

use crate::{
    HeaderType,
    discover::{net_discover_type::NetDiscoverType, partner_service_info::PartnerServiceInfo},
    ffi::FfiBytes,
    lib_app::LIB_APP,
};

pub struct MulticastService {
    sender_ipv4: tokio::net::UdpSocket,
    reciever_ipv4: Vec<Arc<tokio::net::UdpSocket>>,
    multicast_ipv4: Arc<tokio::net::UdpSocket>,
    sender_ipv6: Option<tokio::net::UdpSocket>,
    reciever_ipv6: Vec<Arc<tokio::net::UdpSocket>>,
    multicast_ipv6: Option<Arc<tokio::net::UdpSocket>>,
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
        let temp_handle = handle.clone();
        temp_handle.block_on(async move {
            let mut service_info = PartnerServiceInfo::new();
            let ips = &service_info.net_ips;
            let has_ipv6 = ips
                .iter()
                .any(|ip| ip.ip_v6_link_local.is_unspecified() || ip.ip_v6_unique_local.is_unspecified());
            let multicast_ipv4 = {
                let udp_socket = {
                    let listen_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, Self::MULTICAST_GROUP_PORT);
                    let socket = socket2::Socket::new(socket2::Domain::IPV4, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
                    socket.set_reuse_address(true)?;
                    #[cfg(not(windows))]
                    socket.set_reuse_port(true)?;
                    socket.bind(&listen_addr.into())?;
                    socket.set_nonblocking(true)?;
                    let udp_socket = tokio::net::UdpSocket::from_std(socket.into())?;
                    udp_socket
                };

                udp_socket.join_multicast_v4(*Self::MULTICAST_GROUP_IPV4.ip(), Ipv4Addr::UNSPECIFIED)?;
                log::info!("Successfully connected to multicast server: {}", Self::MULTICAST_GROUP_IPV4);
                Arc::new(udp_socket)
            };

            let mut reciever_ipv4 = Vec::with_capacity(ips.len());
            let mut reciever_ipv6 = Vec::with_capacity(ips.len());
            for ip in &service_info.net_ips {
                let ipv4 = {
                    let mut port = ip.port_v4.load(core::sync::atomic::Ordering::Relaxed);
                    let socket = loop {
                        match std::net::UdpSocket::bind(core::net::SocketAddrV4::new(ip.ip_v4, port)) {
                            Err(_e) => {
                                port += 1;
                                continue;
                            }
                            Ok(s) => {
                                ip.port_v4.store(port, core::sync::atomic::Ordering::Relaxed);
                                break s;
                            }
                        }
                    };
                    socket.set_multicast_loop_v4(false)?;
                    // 设置更大的值（如 32）可以允许消息被路由器转发 (如果路由器配置允许)。
                    socket.set_multicast_ttl_v4(32)?;
                    socket.set_nonblocking(true)?;
                    let socket = tokio::net::UdpSocket::from_std(socket.into())?;
                    log::info!("[Sender] bind: {:?}", socket.local_addr()?);
                    socket
                };
                reciever_ipv4.push(Arc::new(ipv4));

                let ipv6 = {
                    let mut port = ip.port_v6_link_local.load(core::sync::atomic::Ordering::Relaxed);
                    let socket = loop {
                        match std::net::UdpSocket::bind(core::net::SocketAddrV6::new(ip.ip_v6_link_local, port, 0, ip.scope_v6)) {
                            Err(_e) => {
                                port += 1;
                                continue;
                            }
                            Ok(s) => {
                                ip.port_v6_link_local.store(port, core::sync::atomic::Ordering::Relaxed);
                                break s;
                            }
                        }
                    };
                    socket.set_multicast_loop_v6(false)?;
                    socket.set_nonblocking(true)?;
                    let socket = tokio::net::UdpSocket::from_std(socket.into())?;
                    log::info!("[Sender] bind: {:?}", socket.local_addr()?);
                    socket
                };
                reciever_ipv6.push(Arc::new(ipv6));
            }

            let sender_ipv4 = {
                let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
                socket.set_multicast_loop_v4(false)?;
                // 设置更大的值（如 32）可以允许消息被路由器转发 (如果路由器配置允许)。
                socket.set_multicast_ttl_v4(32)?;
                socket.set_nonblocking(true)?;
                let socket = tokio::net::UdpSocket::from_std(socket.into())?;
                log::info!("[Sender] bind: {:?}", socket.local_addr()?);
                socket
            };

            let multicast_ipv6 = {
                if has_ipv6 {
                    let listen_addr = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, Self::MULTICAST_GROUP_PORT, 0, 0);
                    let socket = socket2::Socket::new(socket2::Domain::IPV6, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
                    socket.set_reuse_address(true)?;
                    #[cfg(not(windows))]
                    socket.set_reuse_port(true)?;
                    socket.bind(&listen_addr.into())?;
                    socket.set_nonblocking(true)?;
                    let udp_socket = tokio::net::UdpSocket::from_std(socket.into())?;
                    udp_socket.join_multicast_v6(Self::MULTICAST_GROUP_IPV6.ip(), 0)?;
                    log::info!("Successfully connected to multicast server: {}", Self::MULTICAST_GROUP_IPV6);
                    Some(Arc::new(udp_socket))
                } else {
                    None
                }
            };

            let sender_ipv6 = {
                if has_ipv6 {
                    let socket = std::net::UdpSocket::bind(":::0")?;
                    socket.set_multicast_loop_v6(false)?;
                    socket.set_nonblocking(true)?;
                    let socket = tokio::net::UdpSocket::from_std(socket.into())?;
                    log::info!("[Sender] bind: {:?}", socket.local_addr()?);
                    Some(socket)
                } else {
                    None
                }
            };

            Ok::<Arc<Self>, anyhow::Error>(Arc::new(Self {
                sender_ipv4,
                reciever_ipv4,
                multicast_ipv4,
                sender_ipv6,
                reciever_ipv6,
                multicast_ipv6: multicast_ipv6,
                service_info,
                handle,
            }))
        })
    }
    pub fn init(self: Arc<Self>, cancel_token: tokio_util::sync::CancellationToken) -> tokio::task::JoinHandle<Result<(), anyhow::Error>> {
        const BUFFER_SIZE: usize = 4096;

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

                match self.sender_ipv4.send_to(buffer, &Self::MULTICAST_GROUP_IPV4).await {
                    Ok(sent) => {
                        log::info!("[Sender] Sent {} bytes to {}", sent, Self::MULTICAST_GROUP_IPV4);
                    }
                    Err(e) => {
                        log::error!("[Sender] Error sending to {}: {}", Self::MULTICAST_GROUP_IPV4, e);
                    }
                }
                if let Some(sender_ipv6) = &self.sender_ipv6 {
                    match sender_ipv6.send_to(buffer, &Self::MULTICAST_GROUP_IPV6).await {
                        Ok(sent) => {
                            log::info!("[Sender] Sent {} bytes to {}", sent, Self::MULTICAST_GROUP_IPV6);
                        }
                        Err(e) => {
                            log::error!("[Sender] Error sending to {}: {}", Self::MULTICAST_GROUP_IPV6, e);
                        }
                    }
                }
            }
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
                let mut join_set = tokio::task::JoinSet::new();
                for it in &self.reciever_ipv4 {
                    let it = it.clone();
                    join_set.spawn(async move {
                        let mut buf = vec![0u8; BUFFER_SIZE];
                        let data = it.recv_from(&mut buf).await;
                        match data {
                            Err(e) => {
                                log::error!("Error receiving data on ipv4: {}", e);
                                Err(e)
                            }
                            Ok((len, addr)) => {
                                buf.truncate(len);
                                Ok((addr, buf))
                            }
                        }
                    });
                }
                for it in &self.reciever_ipv6 {
                    let it = it.clone();
                    join_set.spawn(async move {
                        let mut buf = vec![0u8; BUFFER_SIZE];
                        let data = it.recv_from(&mut buf).await;
                        match data {
                            Err(e) => {
                                log::error!("Error receiving data on ipv6: {}", e);
                                Err(e)
                            }
                            Ok((len, addr)) => {
                                buf.truncate(len);
                                Ok((addr, buf))
                            }
                        }
                    });
                }
                let multicast_ipv4 = self.multicast_ipv4.clone();
                join_set.spawn(async move {
                    let mut buf = vec![0u8; BUFFER_SIZE];
                    let data = multicast_ipv4.recv_from(&mut buf).await;
                    match data {
                        Err(e) => {
                            log::error!("Error receiving multicast_ipv4 : {}", e);
                            Err(e)
                        }
                        Ok((len, addr)) => {
                            buf.truncate(len);
                            Ok((addr, buf))
                        }
                    }
                });
                if let Some(receiver_ipv6) = &self.multicast_ipv6 {
                    let receiver_ipv6 = receiver_ipv6.clone();
                    join_set.spawn(async move {
                        let mut buf = vec![0u8; BUFFER_SIZE];
                        let data = receiver_ipv6.recv_from(&mut buf).await;
                        match data {
                            Err(e) => {
                                log::error!("Error receiving multicast_ipv6: {}", e);
                                Err(e)
                            }
                            Ok((len, addr)) => {
                                buf.truncate(len);
                                Ok((addr, buf))
                            }
                        }
                    });
                }

                let re = tokio::select! {
                    data_ipv4 = join_set.join_next() => {
                        data_ipv4
                    }
                    _ = cancel_token.cancelled() => {
                        None
                    }
                };
                if let Some(data) = re {
                    match data {
                        Ok(Ok((addr, buf))) => {
                            err_count = 0;
                            self.handle_recv(addr, buf);
                        }
                        Ok(Err(_e)) => {
                            err_count += 1;
                        }
                        Err(e) => {
                            log::error!("A task panicked or was cancelled: {}", e);
                            err_count += 1;
                        }
                    }
                }
            }
            Ok(())
        })
    }

    fn handle_recv(&self, data: SocketAddr, buffer: Vec<u8>) {
        log::debug!("[Receiver] Received {} bytes from {}", buffer.len(), data);
        if let Some(app) = LIB_APP.get() {
            if let Some(call) = app.get_callback() {
                call(FfiBytes::from(buffer));
            }
        }
    }

    pub fn uninit(&self) -> Result<(), anyhow::Error> {
        self.multicast_ipv4.leave_multicast_v4(Self::MULTICAST_IPV4, Ipv4Addr::UNSPECIFIED)?;
        if let Some(receiver_ipv6) = &self.multicast_ipv6 {
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
