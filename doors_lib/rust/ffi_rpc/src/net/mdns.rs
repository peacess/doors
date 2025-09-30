use std::str::FromStr;
use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig};

pub struct Mdns {
    nets: Vec<NetworkInterface>,
}

impl Mdns {
    pub fn list(&mut self) {
        match NetworkInterface::show() {
            Err(e) => {
                log::error!("Error showing network interfaces: {}", e);
                return;
            },
            Ok(nets) => {
                self.nets.clear();
                'NETS:for net in nets {
                    if let Some(mac) =  &net.mac_addr {
                       if mac == "00:00:00:00:00:00"{
                           continue 'NETS;
                       }
                    }
                    let local = std::net::Ipv4Addr::new(127,0,0,1);
                    let local_v6 = std::net::Ipv6Addr::from_str("::1").unwrap();
                    for addr in &net.addr {
                        match addr {
                            Addr::V4(v4) => {
                                if v4.ip == local {
                                    continue 'NETS;
                                }
                            }
                            Addr::V6(v6) => {
                                if v6.ip == local_v6 {
                                    continue 'NETS;
                                }
                            }
                        }
                    }
                    self.nets.push(net)
                }
            }
        }
    }
}

impl Default for Mdns {
    fn default() -> Self {
        Mdns{
            nets: Vec::new()
        }
    }
}

#[cfg(test)]
mod test{
    use crate::net::mdns::Mdns;

    #[test]
    fn test_mdns() {
        let mut mdns = Mdns::default();
        mdns.list();
    }
}

