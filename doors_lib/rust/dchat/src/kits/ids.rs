use x25519_dalek::{EphemeralSecret, StaticSecret};

pub fn generate_static_secret() -> StaticSecret {
    StaticSecret::random()
}

pub fn generate_ephemeral_secret() -> EphemeralSecret {
    EphemeralSecret::random()
}
