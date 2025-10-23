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
    reciever_ipv4: Arc<tokio::net::UdpSocket>,
    multicast_ipv4: Arc<tokio::net::UdpSocket>,
    multicast_ipv6: Option<Arc<tokio::net::UdpSocket>>,
    service_info: PartnerServiceInfo,
    handle: Handle,
}

impl MulticastService {
    // ipv4 multicast： 239.255.0.0 - 239.255.255.255
    // ipv6 multicast： FF05::/16
    // const GROUP_IPV4: &'static str = "239.0.0.66:5996";
    // const GROUP_IPV6: &'static str = "FF05::66:5996";
    const MULTICAST_PORT_IPV4: u16 = 55996;
    const MULTICAST_PORT_IPV6: u16 = 55997;

    /// 224.0.0.0/24  Local Network Control Block
    /// 224.0.1.0/24  Internetwork Control Block
    /// 239.0.0.0/8  Organization-Local
    const MULTICAST_ADDRV4: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(239, 0, 0, 66), Self::MULTICAST_PORT_IPV4);
    /// ff01: interface-local multicast address
    /// ff02: link-local multicast address
    /// ff04: admin-local multicast address
    /// ff05: site-local multicast address
    /// ff08: organization-local multicast address
    /// ff0E: global multicast address
    const MULTICAST_ADDRV6: Ipv6Addr = Ipv6Addr::new(0xFF05, 0, 0, 0, 0, 0, 0, 66);

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
                    let listen_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, Self::MULTICAST_PORT_IPV4);
                    let socket = socket2::Socket::new(socket2::Domain::IPV4, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
                    socket.set_reuse_address(true)?;
                    socket.set_reuse_port(true)?;
                    socket.set_nonblocking(true)?;
                    // if it false, dont work for two program run in same pc
                    socket.set_multicast_loop_v4(true)?;
                    socket.bind(&listen_addr.into())?;
                    let udp_socket = tokio::net::UdpSocket::from_std(socket.into())?;
                    udp_socket
                };

                udp_socket.join_multicast_v4(*Self::MULTICAST_ADDRV4.ip(), Ipv4Addr::UNSPECIFIED)?;
                log::info!(
                    "Successfully connected to multicast ipv4: {} -- {}",
                    Self::MULTICAST_ADDRV4,
                    udp_socket.local_addr()?
                );
                Arc::new(udp_socket)
            };

            let reciever_ipv4 = {
                let mut port = service_info.port_v4.load(core::sync::atomic::Ordering::Relaxed);
                let reciever_ipv4 = loop {
                    match tokio::net::UdpSocket::bind(core::net::SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port)).await {
                        Err(_e) => {
                            port += 1;
                            continue;
                        }
                        Ok(s) => {
                            service_info.port_v4.store(port, core::sync::atomic::Ordering::Relaxed);
                            break s;
                        }
                    }
                };
                reciever_ipv4.set_multicast_loop_v4(false)?;
                // 设置更大的值（如 32）可以允许消息被路由器转发 (如果路由器配置允许)。
                // socket.set_multicast_ttl_v4(32)?;
                log::info!("[Sender] bind: {:?}", reciever_ipv4.local_addr()?);
                Arc::new(reciever_ipv4)
            };

            for net_ip in &mut service_info.net_ips {
                let sender = {
                    let listen_addr = SocketAddrV4::new(net_ip.ip_v4, 0);
                    let socket = tokio::net::UdpSocket::bind(listen_addr).await?;
                    socket.set_multicast_ttl_v4(32)?;
                    log::info!("[Sender] bind: {:?}", socket.local_addr()?);
                    Arc::new(socket)
                };
                net_ip.sender_ipv4 = Some(sender);
            }

            let multicast_ipv6 = {
                if has_ipv6 {
                    let listen_addr = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, Self::MULTICAST_PORT_IPV6, 0, 0);
                    let socket = socket2::Socket::new(socket2::Domain::IPV6, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
                    socket.set_reuse_address(true)?;
                    socket.set_reuse_port(true)?;
                    socket.set_nonblocking(true)?;
                    socket.set_only_v6(true)?; //dont include the ipv4, for the multicast , ipv6 ipv4 are not same, so dont include
                    socket.bind(&listen_addr.into())?;
                    let udp_socket = tokio::net::UdpSocket::from_std(socket.into())?;
                    for net in &service_info.net_ips {
                        udp_socket.join_multicast_v6(&Self::MULTICAST_ADDRV6, net.index_netinterface)?;
                    }
                    udp_socket.set_multicast_loop_v6(true)?;

                    log::info!(
                        "Successfully connected to multicast ipv6: {} -- {}",
                        Self::MULTICAST_ADDRV6,
                        udp_socket.local_addr()?
                    );
                    Some(Arc::new(udp_socket))
                } else {
                    None
                }
            };

            for net_ip in &mut service_info.net_ips {
                if !net_ip.ip_v6_global.is_unspecified() {
                    let mut port = net_ip.recv_port_v6.load(core::sync::atomic::Ordering::Relaxed);
                    let socket = loop {
                        match tokio::net::UdpSocket::bind(core::net::SocketAddrV6::new(net_ip.ip_v6_global, port, 0, net_ip.index_netinterface)).await {
                            Err(_e) => {
                                port += 1;
                                continue;
                            }
                            Ok(s) => {
                                net_ip.recv_port_v6.store(port, core::sync::atomic::Ordering::Relaxed);
                                break s;
                            }
                        }
                    };
                    socket.set_multicast_loop_v6(true)?;
                    log::info!("receiver ipv6 bind: {}", socket.local_addr()?);
                    net_ip.reciver_ipv6 = Some(Arc::new(socket));
                }
            }

            for net_ip in &mut service_info.net_ips {
                if !net_ip.ip_v6_global.is_unspecified() {
                    let listen_addr = SocketAddrV6::new(net_ip.ip_v6_global, 0, 0, net_ip.index_netinterface);
                    let socket = tokio::net::UdpSocket::bind(listen_addr).await?;
                    log::info!("Sender ipv6 bind: {:?}", socket.local_addr()?);
                    net_ip.sender_ipv6 = Some(Arc::new(socket));
                }
            }

            Ok::<Arc<Self>, anyhow::Error>(Arc::new(Self {
                reciever_ipv4,
                multicast_ipv4,
                multicast_ipv6,
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
                {
                    let hi = {
                        let show_name = builder.create_string(&self.service_info.host_name);
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
                };

                let self_clone = self.clone();
                self.handle.clone().spawn(async move {
                    let buffer = builder.finished_data();
                    for net_ip in &self_clone.service_info.net_ips {
                        if let Some(sender_ipv4) = &net_ip.sender_ipv4 {
                            match sender_ipv4.send_to(buffer, &Self::MULTICAST_ADDRV4).await {
                                Ok(sent) => {
                                    log::debug!(
                                        "ipv4 Sent {} bytes from {} to {}",
                                        sent,
                                        sender_ipv4.local_addr().unwrap(),
                                        Self::MULTICAST_ADDRV4
                                    );
                                }
                                Err(e) => {
                                    log::error!("ipv4 Error sending to {}: {}", Self::MULTICAST_ADDRV4, e);
                                }
                            }
                        }
                    }

                    if self_clone.multicast_ipv6.is_some() {
                        log::debug!("self_clone.multicast_ipv6.is_some");
                        for net_ip in &self_clone.service_info.net_ips {
                            if let Some(sender_ipv6) = &net_ip.sender_ipv6 {
                                let temp = SocketAddrV6::new(Self::MULTICAST_ADDRV6, Self::MULTICAST_PORT_IPV6, 0, net_ip.index_netinterface);
                                match sender_ipv6.send_to(buffer, &temp).await {
                                    Ok(sent) => {
                                        log::debug!(
                                            "ipv6 Sent {} bytes from {} to {}",
                                            sent,
                                            sender_ipv6.local_addr().unwrap(),
                                            Self::MULTICAST_ADDRV6
                                        );
                                    }
                                    Err(e) => {
                                        log::error!("ipv6 Error sending to {}: {}", Self::MULTICAST_ADDRV6, e);
                                    }
                                }
                            }
                        }
                    }
                });
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
                {
                    let it_clone = self.reciever_ipv4.clone();
                    join_set.spawn(async move {
                        let mut buf = vec![0u8; BUFFER_SIZE];
                        let data = it_clone.recv_from(&mut buf).await;
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
                for net_ip in &self.service_info.net_ips {
                    if let Some(reciver_ipv6) = &net_ip.reciver_ipv6 {
                        let it_clone = reciver_ipv6.clone();
                        join_set.spawn(async move {
                            let mut buf = vec![0u8; BUFFER_SIZE];
                            let data = it_clone.recv_from(&mut buf).await;
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
        log::debug!("Received {} bytes from {}", buffer.len(), data);
        //if the "hi" message is send by itself, ignore it
        if data == self.multicast_ipv4.local_addr().unwrap() {
            return;
        }
        if let Some(multicast_ipv6) = &self.multicast_ipv6
            && multicast_ipv6.local_addr().unwrap() == data
        {
            return;
        }
        if let Some(app) = LIB_APP.get() {
            if let Some(call) = app.get_callback() {
                call(FfiBytes::from(buffer));
            }
        }
    }

    pub fn uninit(&self) -> Result<(), anyhow::Error> {
        self.multicast_ipv4.leave_multicast_v4(*Self::MULTICAST_ADDRV4.ip(), Ipv4Addr::UNSPECIFIED)?;
        if let Some(receiver_ipv6) = &self.multicast_ipv6 {
            receiver_ipv6.leave_multicast_v6(&Self::MULTICAST_ADDRV6, 0)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        net::{Ipv4Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
        sync::Arc,
        time::Duration,
    };

    use super::MulticastService;

    #[tokio::test]
    async fn test_tokio_select() {
        let _ = env_logger::builder().is_test(false).filter_level(log::LevelFilter::Debug).try_init();
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

    #[test]
    fn test_multicast_service() -> Result<(), anyhow::Error> {
        let _ = env_logger::builder().is_test(false).filter_level(log::LevelFilter::Debug).try_init();
        let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build()?;
        let multicast_service = MulticastService::new(runtime.handle().clone()).unwrap();
        runtime.block_on(async {
            let cancel_token = tokio_util::sync::CancellationToken::new();
            let _re = multicast_service.clone().init(cancel_token.clone());
            tokio::time::sleep(Duration::from_secs(2)).await;

            for net_ip in &multicast_service.service_info.net_ips {
                if let Some(sender_ipv4) = &net_ip.sender_ipv4 {
                    let re = sender_ipv4.send_to("6test".as_bytes(), &MulticastService::MULTICAST_ADDRV4).await;
                    log::debug!("send re: {:?}", re);
                }
            }

            for net_ip in &multicast_service.service_info.net_ips {
                if let Some(sender_ipv6) = &net_ip.sender_ipv6 {
                    let temp = SocketAddrV6::new(MulticastService::MULTICAST_ADDRV6, MulticastService::MULTICAST_PORT_IPV6, 0, 0);
                    let re = sender_ipv6.send_to("6999test".as_bytes(), &temp).await;
                    log::debug!("send re: {:?}", re);
                }
            }

            tokio::time::sleep(Duration::from_secs(3)).await;
            cancel_token.cancel();
        });

        Ok(())
    }

    #[test]
    fn test_multicast_sample() -> Result<(), anyhow::Error> {
        let _ = env_logger::builder().is_test(false).filter_level(log::LevelFilter::Debug).try_init();
        let runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(4).enable_all().build()?;
        let _ = runtime.block_on(async {
            let socket = {
                let udp_socket = {
                    let listen_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, MulticastService::MULTICAST_PORT_IPV4);
                    let socket = socket2::Socket::new(socket2::Domain::IPV4, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
                    // socket.set_reuse_address(true)?;
                    // #[cfg(not(windows))]
                    // socket.set_reuse_port(true)?;
                    socket.set_multicast_loop_v4(false)?;
                    socket.bind(&listen_addr.into())?;
                    socket.set_nonblocking(true)?;
                    let udp_socket = tokio::net::UdpSocket::from_std(socket.into())?;
                    udp_socket
                };

                udp_socket.join_multicast_v4(*MulticastService::MULTICAST_ADDRV4.ip(), Ipv4Addr::UNSPECIFIED)?;
                log::info!("Successfully connected to multicast server: {}", MulticastService::MULTICAST_ADDRV4);
                Arc::new(udp_socket)
            };

            let socket_clone = socket.clone();

            let recv = tokio::spawn(async move {
                let mut buf = vec![0u8; 4096];
                loop {
                    let data = socket_clone.recv_from(&mut buf).await;
                    match data {
                        Err(e) => {
                            log::error!("Error receiving data on ipv4: {}", e);
                            break;
                        }
                        Ok((len, addr)) => {
                            buf.truncate(len);
                            log::debug!("[Receiver] Received {} bytes from {}", buf.len(), addr);
                            break;
                        }
                    }
                }
            });

            tokio::time::sleep(Duration::from_secs(2)).await;
            let socket_send = {
                let udp_socket = {
                    let listen_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, MulticastService::MULTICAST_PORT_IPV4);
                    let socket = socket2::Socket::new(socket2::Domain::IPV4, socket2::Type::DGRAM, Some(socket2::Protocol::UDP))?;
                    // socket.set_reuse_address(true)?;
                    // #[cfg(not(windows))]
                    // socket.set_reuse_port(true)?;
                    socket.set_multicast_loop_v4(false)?;
                    socket.bind(&listen_addr.into())?;
                    socket.set_nonblocking(true)?;
                    let udp_socket = tokio::net::UdpSocket::from_std(socket.into())?;
                    udp_socket
                };

                udp_socket.set_multicast_ttl_v4(1)?;

                // udp_socket.join_multicast_v4(*MulticastService::MULTICAST_GROUP_IPV4.ip(), Ipv4Addr::UNSPECIFIED)?;
                log::info!("Successfully connected to multicast server: {}", MulticastService::MULTICAST_ADDRV4);
                Arc::new(udp_socket)
            };

            let re = socket_send.send_to("6test".as_bytes(), &MulticastService::MULTICAST_ADDRV4).await;
            log::debug!("send re: {:?}", re);
            tokio::time::sleep(Duration::from_secs(3)).await;
            let _ = recv.await;
            Ok::<(), anyhow::Error>(())
        });

        Ok(())
    }

    #[tokio::test]
    async fn test_multicast_tokio_sample() -> Result<(), anyhow::Error> {
        let _ = env_logger::builder().is_test(false).filter_level(log::LevelFilter::Debug).try_init();
        let multicast_ipv4 = Ipv4Addr::new(239, 0, 0, 1);
        let multicast_addr = SocketAddr::from((multicast_ipv4, 5000));
        let bind_addr = SocketAddr::from(([0, 0, 0, 0], 5000));

        // Bind socket to any local address, same port as multicast
        let socket = tokio::net::UdpSocket::bind(bind_addr).await?;
        socket.set_multicast_loop_v4(false)?;
        // Join multicast group on default interface (0.0.0.0)
        socket.join_multicast_v4(multicast_ipv4, Ipv4Addr::UNSPECIFIED)?;

        log::info!("Listening for multicast messages on {}", socket.local_addr()?);

        let recv = tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let (len, addr) = socket.recv_from(&mut buf).await?;
            let msg = String::from_utf8_lossy(&buf[..len]);
            log::info!("Received from {}: {}", addr, msg);
            Ok::<(), anyhow::Error>(())
        });

        {
            let local_addr = SocketAddr::from(([192, 168, 1, 7], 0));

            let socket = tokio::net::UdpSocket::bind(local_addr).await?;
            // Optional: set multicast TTL or interface
            // socket.set_multicast_ttl_v4(1)?;

            let msg = b"Hello from Tokio multicast!";
            socket.send_to(msg, &multicast_addr).await?;
        }

        let _ = recv.await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_udp_tokio_sample() -> Result<(), anyhow::Error> {
        let _ = env_logger::builder().is_test(false).filter_level(log::LevelFilter::Debug).try_init();
        let recv_addr = SocketAddr::from(([192, 168, 1, 7], 5001));
        let socket = tokio::net::UdpSocket::bind(recv_addr).await?;

        log::info!("Listening for udp on {}", socket.local_addr()?);

        let recv = tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let (len, addr) = socket.recv_from(&mut buf).await?;
            let msg = String::from_utf8_lossy(&buf[..len]);
            log::info!("Received from {}: {}", addr, msg);
            Ok::<(), anyhow::Error>(())
        });

        {
            let local_addr = SocketAddr::from(([192, 168, 1, 7], 0));
            let socket = tokio::net::UdpSocket::bind(local_addr).await?;
            // socket.set_multicast_ttl_v4(1)?;
            let msg = b"Hello from Tokio udp!";
            socket.send_to(msg, &recv_addr).await?;
        }

        let _ = recv.await?;

        Ok(())
    }
}
