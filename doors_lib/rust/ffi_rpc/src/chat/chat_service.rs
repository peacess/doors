use std::net::SocketAddr;

use flatbuffers::EndianScalar;
use idl::{Header, chat_generated::chat::ChatType};

use crate::{
    discover::MulticastService,
    ffi_impl::{CallBack, FfiBytes},
};

impl MulticastService {
    pub fn call_chat(&self, header: &Header, data: &[u8]) -> Result<FfiBytes, anyhow::Error> {
        let chat_type = ChatType::from_little_endian(header.frame_type());
        let data = match chat_type {
            ChatType::none => vec![],
            ChatType::text_message => return self.text_message(data),
            ChatType::text_message_ack => {
                unimplemented!()
            }
            _ => {
                log::error!("Unknown chat type: {:?}", chat_type);
                unimplemented!()
            }
        };
        Ok(FfiBytes::from(data))
    }

    fn text_message(&self, bytes: &[u8]) -> Result<FfiBytes, anyhow::Error> {
        todo!()
        // let mut chat_frame = flatbuffers::root::<ChatTextMessage>(bytes)?;
        // let partner ={
        //     if let Some(message) = chat_frame.message() {
        //         //todo  valid the from_partner_id and from_terminal_id
        //         if let Some(to_partner_id) = message.to_partner_id() {
        //             if self.partner_host.partner_id.0 == to_partner_id.to_u128() {
        //                 Some(&self.partner_host)
        //             }else {
        //                 let
        //                 None
        //             }
        //         }else {
        //             None
        //         }
        //     }else {
        //         None
        //     }
        // };
        // if let Some(partner) = partner {
        //
        // }else {
        //     //maybe forward the message,
        //     Err(anyhow::anyhow!("dont find the to partner"))
        // }
    }

    pub fn init_chat(&self, cancel_token: &tokio_util::sync::CancellationToken) -> Result<(), anyhow::Error> {
        let it_clone = self.reciever_ipv4.clone();
        let call_clone = self.call_back.clone();
        let cancel_clone = cancel_token.clone();
        self.handle.spawn(async move {
            let mut buf = vec![0u8; Self::BUFFER_SIZE];
            let mut err_count = 0;
            loop {
                if err_count > 10 {
                    log::error!("Too many errors receiving data on ipv4");
                    break;
                }
                let data = tokio::select! {
                    _ = cancel_clone.cancelled() => {
                        log::info!("Chat service stopped");
                        None
                    },
                    data = it_clone.recv_from(&mut buf) => Some(data)
                };
                match data {
                    None => {
                        break;
                    }
                    Some(Err(_e)) => {
                        err_count += 1;
                    }
                    Some(Ok((len, addr))) => {
                        buf.truncate(len);
                        Self::handle_net_recv(call_clone.as_ref(), addr, buf.clone());
                    }
                }
            }
        });
        for net_ip in &self.partner_host.net_ips {
            if let Some(reciver_ipv6) = &net_ip.reciver_ipv6 {
                let it_clone = reciver_ipv6.clone();
                let call_clone = self.call_back.clone();
                let cancel_clone = cancel_token.clone();
                self.handle.spawn(async move {
                    let mut buf = vec![0u8; Self::BUFFER_SIZE];
                    let mut err_count = 0;
                    loop {
                        let data = tokio::select! {
                            _ = cancel_clone.cancelled() => {
                                log::info!("Cancelled in ipv6");
                                None
                            },
                            data = it_clone.recv_from(&mut buf)=> Some(data),
                        };
                        match data {
                            None => {
                                break;
                            }
                            Some(Err(e)) => {
                                log::error!("Error receiving data on ipv6({}): {}", it_clone.local_addr().unwrap(), e);
                                err_count += 1;
                                if err_count > 10 {
                                    log::error!("Too many errors on ipv6");
                                    break;
                                }
                            }
                            Some(Ok((len, addr))) => {
                                buf.truncate(len);
                                Self::handle_net_recv(call_clone.as_ref(), addr, buf.clone());
                            }
                        }
                    }
                });
            }
        }
        Ok(())
    }

    fn handle_net_recv(call_back: &Option<CallBack>, data: SocketAddr, buffer: Vec<u8>) {
        log::debug!("Received {} bytes from {}", buffer.len(), data);
        if let Some(call_back) = call_back {
            call_back(FfiBytes::from(buffer));
        }
    }
}
