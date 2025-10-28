use flatbuffers::EndianScalar;
use idl::{
    ErrorInfo, ErrorInfoArgs, Header, TerminalId, UlidBytes, X25519Public,
    base_generated::base::HeaderType,
    net_discovery_generated::net_discovery::{DataSelf, DataSelfArgs, DataSelfFrame, DataSelfFrameArgs, MySelfFrame, NetDiscoveryType},
};
use x25519_dalek::PublicKey;

use super::multicast::MulticastService;
use crate::ffi_impl::FfiBytes;

impl MulticastService {
    pub fn call(&self, header: &Header, data: &[u8]) -> Result<FfiBytes, anyhow::Error> {
        let frame_type = NetDiscoveryType::from_little_endian(header.frame_type());
        let data = match frame_type {
            NetDiscoveryType::none => vec![],
            NetDiscoveryType::hi => {
                todo!()
            }
            NetDiscoveryType::my_self => {
                return self.my_self(data);
            }
            NetDiscoveryType::data_self => {
                unimplemented!();
            }
            _ => {
                log::error!("Unknown frame type: {:?}", frame_type);
                vec![]
            }
        };
        Ok(FfiBytes::from(data))
    }

    fn my_self(&self, bytes: &[u8]) -> Result<FfiBytes, anyhow::Error> {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        {
            let data = {
                let mut req_id = None;
                let mut dns_terminal = None;
                let mut error_info = None;
                match flatbuffers::root::<MySelfFrame>(bytes) {
                    Err(e) => {
                        log::error!("{}", e);
                        let message = Some(builder.create_string(e.to_string().as_str()));
                        error_info = Some(ErrorInfo::create(
                            &mut builder,
                            &ErrorInfoArgs {
                                id: Some(&UlidBytes::from(idl::ids::generate_ulid())),
                                req_id: None,
                                code: 1,
                                message,
                            },
                        ));
                    }
                    Ok(req) => {
                        req_id = {
                            if let Some(temp) = req.my_self() {
                                Some(temp.id())
                            } else {
                                log::info!("field req_id dont exist");
                                None
                            }
                        };
                        dns_terminal = Some(self.service_info.to_dns_terminal(&mut builder));
                    }
                }
                DataSelf::create(
                    &mut builder,
                    &DataSelfArgs {
                        id: Some(&UlidBytes::from(idl::ids::generate_ulid())),
                        dns_terminal,
                        error_info,
                        req_id,
                    },
                )
            };

            let header = Header::new(
                data.value() as u32, //todo check the value
                HeaderType::net_discovery.0,
                NetDiscoveryType::hi.0,
                &TerminalId::from(self.service_info.terminal_id),
                &X25519Public::from(&PublicKey::from(&self.service_info.secret)),
            );
            let data_frame = DataSelfFrame::create(
                &mut builder,
                &DataSelfFrameArgs {
                    header: Some(&header),
                    data_self: Some(data),
                },
            );
            builder.finish(data_frame, None);
        };
        // todo how use the builder data, dont copy
        Ok(FfiBytes::from(builder.finished_data().to_vec()))
    }
}
