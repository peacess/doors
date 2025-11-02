use idl::net_discovery_generated::net_discovery::{DnsTerminal, Hi};

use crate::discover::net_ip_guest::NetIpGuest;

pub struct PartnerServiceGuest {
    pub partner_id: ulid::Ulid,
    pub show_name: String,
    pub terminals: Vec<TerminalGuest>,
}

pub struct TerminalGuest {
    pub partner_id: ulid::Ulid,
    pub terminal_id: ulid::Ulid,
    pub host_name: String,
    pub show_name: String,
    pub net_ips: Vec<NetIpGuest>,
}

impl PartnerServiceGuest {
    pub fn from(hi: &Hi) -> PartnerServiceGuest {
        let mut terminals = Vec::with_capacity(1);
        if let Some(terminal) = &hi.dns_terminal() {
            terminals.push(Self::terminal_guest_from(terminal));
        }

        let partner_id = match terminals.first() {
            Some(it) => it.partner_id,
            None => idl::ids::ulid_zero(),
        };
        let show_name = match hi.show_name() {
            None => match terminals.first() {
                Some(it) => it.show_name.clone(),
                None => "".to_string(),
            },
            Some(name) => name.to_string(),
        };

        PartnerServiceGuest {
            partner_id,
            show_name,
            terminals,
        }
    }
    fn terminal_guest_from(terminal: &DnsTerminal) -> TerminalGuest {
        let partner_id = match terminal.partner_id() {
            Some(id) => idl::ids::array_to_ulid(id.0),
            None => idl::ids::ulid_zero(),
        };
        let show_name = match terminal.show_name() {
            None => "".to_string(),
            Some(name) => name.to_string(),
        };
        let host_name = match terminal.host_name() {
            None => "".to_string(),
            Some(name) => name.to_string(),
        };
        let terminal_id = match terminal.terminal_id() {
            None => idl::ids::ulid_zero(),
            Some(id) => idl::ids::array_to_ulid(id.0),
        };

        let mut net_ips = Vec::with_capacity(terminal.net_interfaces().map_or(0, |ips| ips.len()));
        if let Some(nets) = terminal.net_interfaces() {
            for interface in &nets {
                let net_ip = NetIpGuest::from_bytes(&interface);
                net_ips.push(net_ip);
            }
        }

        TerminalGuest {
            partner_id,
            show_name,
            host_name,
            terminal_id,
            net_ips,
        }
    }
}
