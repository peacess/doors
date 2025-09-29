use uuid::Uuid;

use crate::base_generated::base::UByte16;

#[inline]
pub fn generate_id() -> Uuid {
    Uuid::now_v7()
}

pub fn generate_ubyte16() -> UByte16 {
    let id = generate_id();
    UByte16(id.into_bytes())
}
