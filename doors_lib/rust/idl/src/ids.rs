use uuid::Uuid;

#[inline]
pub fn generate_uuid_v7() -> Uuid {
    Uuid::now_v7()
}
// #[inline]
// pub fn uuid_ubyte16(id: &Uuid) -> UByte16 {
//     UByte16(id.into_bytes())
// }

#[inline]
pub fn generate_ulid() -> ulid::Ulid {
    ulid::Ulid::new()
}

pub fn ulid_zero() -> ulid::Ulid {
    ulid::Ulid::from_bytes([0; 16])
}
// #[inline]
// pub fn ulid_ubyte16(id: &ulid::Ulid) -> UByte16 {
//     UByte16(id.to_bytes())
// }

pub fn array_to_ulid(bs: [u8; 16]) -> ulid::Ulid {
    ulid::Ulid::from_bytes(bs)
}
