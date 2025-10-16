use x25519_dalek::EphemeralSecret;

pub struct PartnerServiceInfo {
    pub id: ulid::Ulid,
    pub instance_name: String,
    pub service_type: String,
    pub port: u16,
    pub secret: EphemeralSecret,
}

impl PartnerServiceInfo {
    pub fn new() -> PartnerServiceInfo {
        let secret = EphemeralSecret::random();
        let mut port = 9933u16;
        loop {
            match std::net::UdpSocket::bind("") {
                Err(_e) => {
                    port += 1;
                    continue;
                }
                Ok(_s) => {
                    break;
                }
            }
        }
        PartnerServiceInfo {
            id: idl::ids::generate_ulid(),
            instance_name: "doors_chat".into(),
            service_type: "_http._tcp".into(),
            port,
            secret,
        }
    }
}
