use std::{
    net::{Ipv4Addr, Ipv6Addr},
    sync::{Arc, atomic::Ordering},
};

use flatbuffers::FlatBufferBuilder;
use idl::{
    Ipv6,
    net_discovery_generated::net_discovery::{NetInterface, NetInterfaceArgs, NetInterfaceT},
};

#[derive(Debug)]
pub struct NetIp {
    pub ip_v4: Ipv4Addr,
    pub port_v4: Arc<core::sync::atomic::AtomicU16>,
    pub ip_v6_global: Ipv6Addr,
    pub port_v6_global: Arc<core::sync::atomic::AtomicU16>,
    pub ip_v6_temporary: Ipv6Addr,
    pub port_v6_temporary: Arc<core::sync::atomic::AtomicU16>,
    pub ip_v6_link_local: Ipv6Addr,
    pub port_v6_link_local: Arc<core::sync::atomic::AtomicU16>,
    pub scope_v6: u32,
    pub ip_v6_unique_local: Ipv6Addr,
    pub port_v6_unique_local: Arc<core::sync::atomic::AtomicU16>,
    pub name: String,
    pub mac_address: String,
}

impl NetIp {
    pub const DEFAULT_PORT: u16 = 9933;

    pub fn to_iet_interface<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
        &self,
        builder: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    ) -> flatbuffers::WIPOffset<NetInterface<'bldr>> {
        let name = builder.create_string(&self.name);
        let mac_address = builder.create_string(&self.mac_address);
        NetInterface::create(
            builder,
            &NetInterfaceArgs {
                ip_v4: self.ip_v4.to_bits(),
                port_v4: self.port_v4.load(Ordering::Relaxed),
                ip_v6_global: Some(&Ipv6(self.ip_v6_global.octets())),
                port_v6_global: self.port_v6_global.load(Ordering::Relaxed),
                ip_v6_temporary: Some(&Ipv6(self.ip_v6_temporary.octets())),
                port_v6_temporary: self.port_v6_temporary.load(Ordering::Relaxed),
                ip_v6_link_local: Some(&Ipv6(self.ip_v6_link_local.octets())),
                port_v6_link_local: self.port_v6_link_local.load(Ordering::Relaxed),
                scope_v6: self.scope_v6,
                ip_v6_unique_local: Some(&Ipv6(self.ip_v6_link_local.octets())),
                port_v6_unique_local: self.port_v6_link_local.load(Ordering::Relaxed),
                name: Some(name),
                mac_address: Some(mac_address),
            },
        )
    }
}

impl Default for NetIp {
    fn default() -> Self {
        Self {
            ip_v4: Ipv4Addr::from_bits(0),
            port_v4: Arc::new(core::sync::atomic::AtomicU16::new(0)),
            ip_v6_global: Ipv6Addr::from_bits(0),
            port_v6_global: Arc::new(core::sync::atomic::AtomicU16::new(0)),
            ip_v6_temporary: Ipv6Addr::from_bits(0),
            port_v6_temporary: Arc::new(core::sync::atomic::AtomicU16::new(0)),
            ip_v6_link_local: Ipv6Addr::from_bits(0),
            port_v6_link_local: Arc::new(core::sync::atomic::AtomicU16::new(0)),
            ip_v6_unique_local: Ipv6Addr::from_bits(0),
            port_v6_unique_local: Arc::new(core::sync::atomic::AtomicU16::new(0)),
            scope_v6: 0,
            name: "".to_string(),
            mac_address: "".to_string(),
        }
    }
}
