#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter)]
pub enum HeaderType {
    NetDiscovery = 1,
    Chat = 2,
}

impl HeaderType {
    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

impl TryFrom<u32> for HeaderType {
    type Error = anyhow::Error;
    fn try_from(value: u32) -> Result<Self, anyhow::Error> {
        match value {
            1 => Ok(HeaderType::NetDiscovery),
            2 => Ok(HeaderType::Chat),
            _ => Err(anyhow::anyhow!("Unknown header type: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::HeaderType;
    #[test]
    fn test_header_type_to_u32() {
        for it in HeaderType::iter() {
            let v = it.to_u32();
            let ev = HeaderType::try_from(v).unwrap();
            assert_eq!(it, ev);
        }
    }
}
