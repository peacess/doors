use std::str::FromStr;

use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig};

pub struct NetHelper {}

impl NetHelper {
    pub fn list() -> Vec<std::net::IpAddr> {
        let mut service_list = Vec::with_capacity(6);
        match NetworkInterface::show() {
            Err(e) => {
                log::error!("Error showing network interfaces: {}", e);
            }
            Ok(nets) => {
                'NETS: for net in nets {
                    if let Some(mac) = &net.mac_addr
                        && mac == "00:00:00:00:00:00"
                    {
                        continue 'NETS;
                    }
                    if net.addr.is_empty() {
                        continue 'NETS;
                    }
                    let local = std::net::Ipv4Addr::new(127, 0, 0, 1);
                    let local_v6 = std::net::Ipv6Addr::from_str("::1").unwrap();
                    for addr in &net.addr {
                        match addr {
                            Addr::V4(v4) => {
                                if v4.ip == local {
                                    continue 'NETS;
                                }
                                service_list.push(std::net::IpAddr::V4(v4.ip));
                            }
                            Addr::V6(v6) => {
                                if v6.ip == local_v6 {
                                    continue 'NETS;
                                }
                                service_list.push(std::net::IpAddr::V6(v6.ip));
                            }
                        }
                    }
                }
            }
        }

        service_list
    }
}
