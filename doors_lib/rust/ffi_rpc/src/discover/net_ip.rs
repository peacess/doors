use std::net::{Ipv4Addr, Ipv6Addr};

use flatbuffers::FlatBufferBuilder;
use idl::{
    Ipv6,
    net_discovery_generated::net_discovery::{NetInterface, NetInterfaceArgs, NetInterfaceT},
};
pub struct NetIp {
    pub ip_v4: Ipv4Addr,
    pub port_v4: u16,
    pub ip_v6_global: Ipv6Addr,
    pub port_v6_global: u16,
    pub ip_v6_temporary: Ipv6Addr,
    pub port_v6_temporary: u16,
    pub ip_v6_link_local: Ipv6Addr,
    pub port_v6_link_local: u16,
    pub scope_v6: u32,
    pub ip_v6_unique_local: Ipv6Addr,
    pub port_v6_unique_local: u16,
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
                port_v4: self.port_v4,
                ip_v6_global: Some(&Ipv6(self.ip_v6_global.octets())),
                port_v6_global: self.port_v6_global,
                ip_v6_temporary: Some(&Ipv6(self.ip_v6_temporary.octets())),
                port_v6_temporary: self.port_v6_temporary,
                ip_v6_link_local: Some(&Ipv6(self.ip_v6_link_local.octets())),
                port_v6_link_local: self.port_v6_link_local,
                scope_v6: self.scope_v6,
                ip_v6_unique_local: Some(&Ipv6(self.ip_v6_link_local.octets())),
                port_v6_unique_local: self.port_v6_link_local,
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
            port_v4: 0,
            ip_v6_global: Ipv6Addr::from_bits(0),
            port_v6_global: 0,
            ip_v6_temporary: Ipv6Addr::from_bits(0),
            port_v6_temporary: 0,
            ip_v6_link_local: Ipv6Addr::from_bits(0),
            port_v6_link_local: 0,
            ip_v6_unique_local: Ipv6Addr::from_bits(0),
            port_v6_unique_local: 0,
            scope_v6: 0,
            name: "".to_string(),
            mac_address: "".to_string(),
        }
    }
}
