use crate::dns_serde::DnsSerialize;

#[derive(Debug, PartialEq)]
pub enum DnsType {
    A(u8, u8, u8, u8),
    _Cname,
}

impl DnsType {
    pub fn int_as_bytes(&self) -> [u8; 2] {
        match self {
            DnsType::A(_, _, _, _) => 1u16,
            DnsType::_Cname => 5u16,
        }
        .to_be_bytes()
    }

    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        match u16::from_be_bytes(bytes) {
            1 => DnsType::A(0, 0, 0, 0),
            5 => DnsType::_Cname,
            0_u16 | 2_u16..=4_u16 | 6_u16..=u16::MAX => todo!(),
        }
    }

    pub fn len_as_bytes(&self) -> [u8; 2] {
        match self {
            DnsType::A(_, _, _, _) => 4u16,
            DnsType::_Cname => todo!(),
        }
        .to_be_bytes()
    }
}

impl DnsSerialize for DnsType {
    fn serialize(&self) -> Vec<u8> {
        match self {
            DnsType::A(a, b, c, d) => vec![*a, *b, *c, *d],
            DnsType::_Cname => todo!(),
        }
    }
}

impl Default for DnsType {
    fn default() -> Self {
        DnsType::A(0, 0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes() {
        assert_eq!(DnsType::A(8, 8, 8, 8).serialize(), [8, 8, 8, 8])
    }

    #[test]
    fn it_returns_correct_length() {
        assert_eq!(DnsType::A(8, 8, 8, 8).len_as_bytes(), [0, 4])
    }

    #[test]
    fn it_returns_correct_type_id() {
        assert_eq!(DnsType::A(8, 8, 8, 8).int_as_bytes(), [0, 1]);
        assert_eq!(DnsType::_Cname.int_as_bytes(), [0, 5]);
    }
}
