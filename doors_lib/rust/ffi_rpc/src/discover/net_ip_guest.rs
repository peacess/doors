use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
pub struct NetIpGuest {
    pub ip_v4: Ipv4Addr,
    pub ip_v6_global: Ipv6Addr,
    pub ip_v6_temporary: Ipv6Addr,
    pub ip_v6_link_local: Ipv6Addr,
    pub ip_v6_unique_local: Ipv6Addr,
    pub recv_port_v4: u16,
    pub recv_port_v6: u16,
    // Internal index identifying netinterface, other name is scope
    pub index_netinterface: u32,
    pub name: String,
    pub mac_address: String,
}

impl NetIpGuest {
    pub fn from_bytes(bs: &idl::net_discovery_generated::net_discovery::NetInterface) -> Self {
        let ip_v4 = Ipv4Addr::from(bs.ip_v4());
        let mut ip_v6_global = Ipv6Addr::UNSPECIFIED;
        if let Some(ip) = bs.ip_v6_global() {
            ip_v6_global = Ipv6Addr::from(ip.0);
        }
        let mut ip_v6_temporary = Ipv6Addr::UNSPECIFIED;
        if let Some(ip) = bs.ip_v6_temporary() {
            ip_v6_temporary = Ipv6Addr::from(ip.0);
        }
        let mut ip_v6_link_local = Ipv6Addr::UNSPECIFIED;
        if let Some(ip) = bs.ip_v6_link_local() {
            ip_v6_link_local = Ipv6Addr::from(ip.0);
        }
        let mut ip_v6_unique_local = Ipv6Addr::UNSPECIFIED;
        if let Some(ip) = bs.ip_v6_unique_local() {
            ip_v6_unique_local = Ipv6Addr::from(ip.0);
        }
        let mut name = String::new();
        if let Some(str) = bs.name() {
            name = str.to_string();
        }
        let mut mac_address = "".to_string();
        if let Some(mac) = bs.mac_address() {
            mac_address = mac.to_string();
        }

        Self {
            ip_v4,
            ip_v6_global,
            ip_v6_temporary,
            ip_v6_link_local,
            ip_v6_unique_local,
            recv_port_v4: bs.port_v4(),
            recv_port_v6: bs.port_v6(),
            index_netinterface: bs.index_netinterface(),
            name,
            mac_address,
        }
    }
}
