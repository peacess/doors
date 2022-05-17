use crate::idl::Int128;

///

pub struct Partner {
    id: Int128,
    terminal_id: Int128,//终端id
    partner_id: Int128,//partner
    name: String,
    ip: String,
    port: i16,
    create_ts: i64,
}



