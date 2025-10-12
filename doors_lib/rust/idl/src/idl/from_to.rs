use crate::idl::{UByte16, UlidBytes, X25519Public};

impl From<ulid::Ulid> for UlidBytes {
    fn from(value: ulid::Ulid) -> Self {
        Self(value.to_bytes())
    }
}

impl From<&x25519_dalek::PublicKey> for X25519Public {
    fn from(value: &x25519_dalek::PublicKey) -> Self {
        Self(value.to_bytes())
    }
}
