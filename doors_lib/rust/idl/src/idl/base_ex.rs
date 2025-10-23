use super::base_generated::base::UlidBytes;

impl UlidBytes {
    pub fn to_u128(&self) -> u128 {
        ulid::Ulid::from_bytes(self.0).0
    }
}
