use std::{ffi::CStr, net::IpAddr};

use idl::{
    PartnerId, TerminalId, X25519Public,
    net_discovery_generated::net_discovery::{DnsTerminal, DnsTerminalArgs, NetInterface},
};
use x25519_dalek::{EphemeralSecret, PublicKey};

use super::net_ip::NetIp;

pub struct PartnerServiceInfo {
    pub partner_id: ulid::Ulid,
    pub instance_name: String,
    pub host_name: String,
    pub service_type: String,
    pub secret: EphemeralSecret,
    pub terminal_id: ulid::Ulid,
    pub net_ips: Vec<NetIp>,
}

impl PartnerServiceInfo {
    pub fn new() -> PartnerServiceInfo {
        let secret = EphemeralSecret::random();

        let mut net_ips = Vec::new();
        match netwatcher::list_interfaces() {
            Err(e) => {
                log::error!("Error showing network interfaces: {}", e);
            }
            Ok(nets) => {
                for net in nets {
                    if let Some(ip) = Self::net_ip_from_net(net.1) {
                        net_ips.push(ip);
                    }
                }
            }
        }
        let host_name = {
            let mut buf = [0u8; 512];
            let re = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut i8, buf.len() - 1) };
            if re == 0 {
                unsafe { CStr::from_ptr(buf.as_ptr() as *const i8).to_string_lossy().into_owned() }
            } else {
                log::error!("gethostname returned an error:{}", re);
                "".to_string()
            }
        };

        PartnerServiceInfo {
            partner_id: idl::ids::generate_ulid(),
            instance_name: "doors_chat".into(),
            host_name,
            service_type: "_http._tcp".into(),
            secret,
            terminal_id: idl::ids::generate_ulid(),
            net_ips,
        }
    }

    fn net_ip_from_net(net: netwatcher::Interface) -> Option<NetIp> {
        if net.ips.is_empty() || net.hw_addr == "00:00:00:00:00:00" {
            return None;
        }

        let mut net_ip = NetIp::default();
        net_ip.name = net.name;
        net_ip.mac_address = net.hw_addr;
        net_ip.scope_v6 = net.index;
        for add in net.ips {
            match add.ip {
                IpAddr::V4(v4) => {
                    log::debug!("{}", v4);
                    if v4.is_loopback() {
                        continue;
                    }
                    net_ip.ip_v4 = v4;
                    {
                        let mut port = NetIp::DEFAULT_PORT;
                        loop {
                            match std::net::UdpSocket::bind(core::net::SocketAddrV4::new(net_ip.ip_v4, port)) {
                                Err(_e) => {
                                    port += 1;
                                    continue;
                                }
                                Ok(_s) => {
                                    net_ip.port_v4 = port;
                                    break;
                                }
                            }
                        }
                    }
                }
                IpAddr::V6(v6) => {
                    log::debug!("{}", v6);
                    if v6.is_loopback() {
                        continue;
                    }

                    if v6.is_unicast_link_local() {
                        let mut port = NetIp::DEFAULT_PORT;
                        loop {
                            match std::net::UdpSocket::bind(core::net::SocketAddrV6::new(v6, port, 0, net.index)) {
                                Err(e) => {
                                    log::debug!("{}", e);
                                    port += 1;
                                    continue;
                                }
                                Ok(_s) => {
                                    net_ip.port_v4 = port;
                                    break;
                                }
                            }
                        }
                        net_ip.ip_v6_link_local = v6;
                        net_ip.port_v6_link_local = port;
                    } else if v6.is_unique_local() {
                        let mut port = NetIp::DEFAULT_PORT;
                        loop {
                            match std::net::UdpSocket::bind(core::net::SocketAddrV6::new(v6, port, 0, net.index)) {
                                Err(e) => {
                                    log::debug!("{}", e);
                                    port += 1;
                                    continue;
                                }
                                Ok(_s) => {
                                    net_ip.port_v4 = port;
                                    break;
                                }
                            }
                        }
                        net_ip.ip_v6_unique_local = v6;
                        net_ip.port_v6_unique_local = port;
                    } else {
                        let mut port = NetIp::DEFAULT_PORT;
                        loop {
                            match std::net::UdpSocket::bind(core::net::SocketAddrV6::new(v6, port, 0, 0)) {
                                Err(e) => {
                                    log::debug!("{}", e);
                                    port += 1;
                                    continue;
                                }
                                Ok(_s) => {
                                    net_ip.port_v4 = port;
                                    break;
                                }
                            }
                        }
                        if net_ip.ip_v6_global.is_unspecified() {
                            net_ip.ip_v6_global = v6;
                            net_ip.port_v6_global = port;
                        } else {
                            net_ip.ip_v6_temporary = v6;
                            net_ip.port_v6_temporary = port;
                        }
                    }
                }
            }
        }
        Some(net_ip)
    }

    pub fn to_dns_terminal<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
        &self,
        builder: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    ) -> flatbuffers::WIPOffset<DnsTerminal<'bldr>> {
        let host_name = builder.create_string(&self.host_name);
        let show_name = builder.create_string("doors_chat");
        let net_interfaces = self.net_ips.iter().map(|net| net.to_iet_interface(builder)).collect::<Vec<_>>();
        let net_interfaces = builder.create_vector(&net_interfaces);
        DnsTerminal::create(
            builder,
            &DnsTerminalArgs {
                partner_id: Some(&PartnerId::from(self.partner_id)),
                terminal_id: Some(&TerminalId::from(self.terminal_id)),
                key: Some(&X25519Public::from(&PublicKey::from(&self.secret))),
                host_name: Some(host_name),
                show_name: Some(show_name),
                net_interfaces: Some(net_interfaces),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::discover::partner_service_info::PartnerServiceInfo;

    #[test]
    fn test_net_ip_from_net() {
        env_logger::builder().is_test(false).filter_level(log::LevelFilter::Debug).init();
        let _net_ip = PartnerServiceInfo::new();
    }
}
