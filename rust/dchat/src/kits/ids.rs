use uuid::Uuid;
use crate::idl::Int128;

#[inline]
pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}

pub fn generate_int128() -> Int128{
    let id = generate_id();
    let mut re = Int128::default();
    re.0.copy_from_slice(id.as_ref());
    re
}
