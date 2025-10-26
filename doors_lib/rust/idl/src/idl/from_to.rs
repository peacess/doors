use crate::{
    PartnerId, TerminalId,
    idl::{UlidBytes, X25519Public},
    partner_generated::partner::Partner,
};

impl From<ulid::Ulid> for UlidBytes {
    fn from(value: ulid::Ulid) -> Self {
        Self(value.to_bytes())
    }
}

impl From<ulid::Ulid> for PartnerId {
    fn from(value: ulid::Ulid) -> Self {
        Self(value.to_bytes())
    }
}

impl From<ulid::Ulid> for TerminalId {
    fn from(value: ulid::Ulid) -> Self {
        Self(value.to_bytes())
    }
}

impl From<&x25519_dalek::PublicKey> for X25519Public {
    fn from(value: &x25519_dalek::PublicKey) -> Self {
        Self(value.to_bytes())
    }
}
