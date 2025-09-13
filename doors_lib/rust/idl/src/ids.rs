use uuid::Uuid;

use crate::base_generated::base::UByte16;

#[inline]
pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

pub fn generate_int128() -> UByte16 {
    let id = generate_id();
    let mut re = UByte16::default();
    re.0.copy_from_slice(id.as_ref());
    re
}
