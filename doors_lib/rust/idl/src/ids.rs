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
// #[inline]
// pub fn ulid_ubyte16(id: &ulid::Ulid) -> UByte16 {
//     UByte16(id.to_bytes())
// }
