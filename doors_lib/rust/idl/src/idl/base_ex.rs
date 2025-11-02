use super::base_generated::base::{Header, HeaderType, PartnerId, TerminalId, UlidBytes, X25519Public};

impl UlidBytes {
    pub fn to_u128(&self) -> u128 {
        ulid::Ulid::from_bytes(self.0).0
    }
}

impl Header {
    pub fn zero_error(len: u32) -> Self {
        Header::new(len, HeaderType::error_info.0, 0, &TerminalId::zero(), &X25519Public::zero())
    }
}

impl PartnerId {
    pub fn zero() -> Self {
        PartnerId::new(0, 0)
    }
    pub fn to_u128(&self) -> u128 {
        ulid::Ulid::from_bytes(self.0).0
    }
}

impl TerminalId {
    pub fn zero() -> Self {
        TerminalId::new(0, 0)
    }
    pub fn to_u128(&self) -> u128 {
        ulid::Ulid::from_bytes(self.0).0
    }
}

impl X25519Public {
    pub fn zero() -> Self {
        X25519Public::new(0, 0, 0, 0)
    }
}
