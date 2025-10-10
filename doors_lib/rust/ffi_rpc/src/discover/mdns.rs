use std::{str::FromStr, sync::Arc};

use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig};

use crate::discover::{multicast::MulticastService, partner_service_info::PartnerServiceInfo};

pub struct Mdns {
    nets: Vec<NetworkInterface>,
    len_addr: usize,
    service_info: PartnerServiceInfo,
    multicast_service: Arc<MulticastService>,
}

impl Mdns {
    pub fn new() -> Result<Self, anyhow::Error> {
        Ok(Self {
            nets: Vec::new(),
            len_addr: 0,
            service_info: PartnerServiceInfo::new(),
            multicast_service: MulticastService::new(vec![])?,
        })
    }
    pub fn list(&mut self) {
        match NetworkInterface::show() {
            Err(e) => {
                log::error!("Error showing network interfaces: {}", e);
            }
            Ok(nets) => {
                self.nets.clear();
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
                            }
                            Addr::V6(v6) => {
                                if v6.ip == local_v6 {
                                    continue 'NETS;
                                }
                            }
                        }
                    }
                    self.len_addr = net.addr.len();
                    self.nets.push(net);
                }
            }
        }
    }
    pub fn register_service(&mut self) -> Result<(), anyhow::Error> {
        // let mut addrs = Vec::<std::net::IpAddr>::with_capacity(self.nets.len() *2);
        // for net in &self.nets {
        //     for addr in &net.addr {
        //         match addr {
        //             Addr::V4(v4) => {
        //                 addrs.push(std::net::IpAddr::V4(v4.ip));
        //             },
        //             Addr::V6(v6) => {
        //                 addrs.push(std::net::IpAddr::V6(v6.ip));
        //             }
        //         }
        //     }
        // }
        // let responder = libmdns::Responder::new_with_ip_list(vec![])?;
        // let service = responder.register_with_ttl(&self.service_info.service_type, &self.service_info.instance_name, self.service_info.port,&[""],1024);
        // Ok(service)
        Ok(())
    }

    pub fn discover_service(&mut self) -> Result<(), anyhow::Error> {
        // let service = mdns_sd::ServiceDaemon::new()?;
        // let discover = service.browse("._udp.local.")?;
        // loop {
        //     match discover.recv_timeout(Duration::from_secs(3)) {
        //         Ok(event) => {
        //             log::debug!("{:?}", event);
        //             match event {
        //                 ServiceEvent::ServiceResolved(service) => {
        //                     log::info!("Discovered service: {:?}", service.fullname);
        //                 }
        //                 ServiceEvent::ServiceRemoved(service_type, name) => {
        //                     log::info!("Resolved service: {:?}", name);
        //                 }
        //                 _ =>{     }
        //             }
        //         }
        //         Err(e) => {
        //             log::debug!("Error discovering services: {}", e);
        //         }
        //     }
        // }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    // use crate::net::mdns::Mdns;
    //
    // #[tokio::test]
    // async  fn test_mdns() {
    //     env_logger::builder().is_test(true).filter_level(log::LevelFilter::Debug).init();
    //     let mut mdns = Mdns::default();
    //     mdns.list();
    //     let service = mdns.register_service();
    //     // let re = mdns.discover_service_rd();
    //     // if let Err(e) = re {
    //     //     log::error!("Error discovering services: {}", e);
    //     // }
    //
    //     let service = libmdns::Responder::new();
    //     let discover = service.browse(&mdns.service_info.service_type);
    //     while let Ok(service) = discover.recv().await {
    //         log::info!("Discovered service: {:?}", service);
    //     }
    // }
}
