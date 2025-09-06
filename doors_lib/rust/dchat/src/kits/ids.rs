use uuid::Uuid;
use x25519_dalek::{EphemeralSecret, StaticSecret};

use crate::idl::UByte16;

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

pub fn generate_static_secret() -> StaticSecret {
    StaticSecret::random()
}

pub fn generate_ephemeral_secret() -> EphemeralSecret {
    EphemeralSecret::random()
}
