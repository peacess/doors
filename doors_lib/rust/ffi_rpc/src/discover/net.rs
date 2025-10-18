use std::{net::IpAddr, str::FromStr};

pub struct NetHelper {}

impl NetHelper {
    pub fn list() -> Vec<std::net::IpAddr> {
        let mut service_list = Vec::with_capacity(6);
        match netwatcher::list_interfaces() {
            Err(e) => {
                log::error!("Error showing network interfaces: {}", e);
            }
            Ok(nets) => {
                'NETS: for (_, net) in nets {
                    if net.ips.is_empty() || net.hw_addr == "00:00:00:00:00:00" {
                        continue 'NETS;
                    }
                    for addr in &net.ips {
                        match addr.ip {
                            IpAddr::V4(v4) => {
                                if v4.is_loopback() {
                                    continue 'NETS;
                                }
                                service_list.push(std::net::IpAddr::V4(v4));
                            }
                            IpAddr::V6(v6) => {
                                if v6.is_loopback() {
                                    continue 'NETS;
                                }
                                service_list.push(std::net::IpAddr::V6(v6));
                            }
                        }
                    }
                }
            }
        }

        service_list
    }
}
