use crate::data::Id16;

///

pub struct Partner {
    id: Id16,
    terminal_id: Id16,
    //终端id
    partner_id: Id16,
    //partner
    name: String,
    ip: String,
    port: i32,
    create_ts: i64,
}



