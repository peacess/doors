
pub struct PartnerServiceInfo {
    pub id: u128,
    pub instance_name:String,
    pub service_type: String,
    pub port: u16,
}

impl PartnerServiceInfo {
    pub fn new() -> PartnerServiceInfo {
        PartnerServiceInfo{
            id: idl::ids::generate_id().to_u128_le(),
            instance_name: "doors_chat".into(),
            service_type: "_http._tcp".into(),
            port: 9933,
        }
    }
}