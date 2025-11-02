use std::{
    net::{Ipv4Addr, Ipv6Addr},
    sync::{Arc, atomic::Ordering},
};

use idl::{
    Ipv6,
    net_discovery_generated::net_discovery::{NetInterface, NetInterfaceArgs},
};

#[derive(Debug)]
pub struct NetIpHost {
    pub ip_v4: Ipv4Addr,
    pub ip_v6_global: Ipv6Addr,
    pub ip_v6_temporary: Ipv6Addr,
    pub ip_v6_link_local: Ipv6Addr,
    pub ip_v6_unique_local: Ipv6Addr,
    pub recv_port_v4: Arc<core::sync::atomic::AtomicU16>,
    pub recv_port_v6: Arc<core::sync::atomic::AtomicU16>,
    // Internal index identifying netinterface, other name is scope
    pub index_netinterface: u32,
    pub name: String,
    pub mac_address: String,

    pub sender_ipv4: Option<Arc<tokio::net::UdpSocket>>,
    pub sender_ipv6: Option<Arc<tokio::net::UdpSocket>>,
    pub reciver_ipv6: Option<Arc<tokio::net::UdpSocket>>,
}

impl NetIpHost {
    pub const DEFAULT_PORT: u16 = 59933;

    pub fn to_iet_interface<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
        &self,
        builder: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
        port_v4: u16,
    ) -> flatbuffers::WIPOffset<NetInterface<'bldr>> {
        let name = builder.create_string(&self.name);
        let mac_address = builder.create_string(&self.mac_address);
        NetInterface::create(
            builder,
            &NetInterfaceArgs {
                ip_v4: self.ip_v4.to_bits(),
                port_v4,
                ip_v6_global: Some(&Ipv6(self.ip_v6_global.octets())),
                port_v6: self.recv_port_v6.load(Ordering::Relaxed),
                ip_v6_temporary: Some(&Ipv6(self.ip_v6_temporary.octets())),
                ip_v6_link_local: Some(&Ipv6(self.ip_v6_link_local.octets())),
                index_netinterface: self.index_netinterface,
                ip_v6_unique_local: Some(&Ipv6(self.ip_v6_link_local.octets())),
                name: Some(name),
                mac_address: Some(mac_address),
            },
        )
    }
}

impl Default for NetIpHost {
    fn default() -> Self {
        Self {
            ip_v4: Ipv4Addr::from_bits(0),
            ip_v6_global: Ipv6Addr::from_bits(0),
            ip_v6_temporary: Ipv6Addr::from_bits(0),
            ip_v6_link_local: Ipv6Addr::from_bits(0),
            ip_v6_unique_local: Ipv6Addr::from_bits(0),
            recv_port_v4: Arc::new(core::sync::atomic::AtomicU16::new(0)),
            recv_port_v6: Arc::new(core::sync::atomic::AtomicU16::new(Self::DEFAULT_PORT)),
            index_netinterface: 0,
            name: "".to_string(),
            mac_address: "".to_string(),
            sender_ipv4: None,
            sender_ipv6: None,
            reciver_ipv6: None,
        }
    }
}
