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
        PartnerServiceInfo {
            id: idl::ids::generate_ulid(),
            instance_name: "doors_chat".into(),
            service_type: "_http._tcp".into(),
            port: 9933,
            secret,
        }
    }
}
