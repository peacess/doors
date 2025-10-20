#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter)]
pub enum NetDiscoverType {
    Hi = 1,
}

impl NetDiscoverType {
    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

impl TryFrom<u32> for NetDiscoverType {
    type Error = anyhow::Error;
    fn try_from(value: u32) -> Result<Self, anyhow::Error> {
        match value {
            1 => Ok(Self::Hi),
            _ => Err(anyhow::anyhow!("Unknown net discover type: {}", value)),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter)]
pub enum NetDiscoverCallbackType {
    HiRec = 1,
}

impl NetDiscoverCallbackType {
    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

impl TryFrom<u32> for NetDiscoverCallbackType {
    type Error = anyhow::Error;
    fn try_from(value: u32) -> Result<Self, anyhow::Error> {
        match value {
            1 => Ok(Self::HiRec),
            _ => Err(anyhow::anyhow!("Unknown net discover callback type: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::{NetDiscoverCallbackType, NetDiscoverType};
    #[test]
    fn test_net_discover_type_to_u32() {
        for it in NetDiscoverType::iter() {
            let v = it.to_u32();
            let ev = NetDiscoverType::try_from(v).unwrap();
            assert_eq!(it, ev);
        }
    }

    #[test]
    fn test_net_discover_callback_type_to_u32() {
        for it in NetDiscoverCallbackType::iter() {
            let v = it.to_u32();
            let ev = NetDiscoverCallbackType::try_from(v).unwrap();
            assert_eq!(it, ev);
        }
    }
}
