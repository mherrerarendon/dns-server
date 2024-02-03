use crate::dns_serde::DnsSerialize;

#[derive(Debug, PartialEq, Clone)]
pub enum DnsType {
    A(u8, u8, u8, u8),
    _Cname,
}

impl DnsType {
    pub fn int_as_bytes(&self) -> [u8; 2] {
        match self {
            DnsType::A(..) => 1u16,
            DnsType::_Cname => 5u16,
        }
        .to_be_bytes()
    }

    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        match u16::from_be_bytes(bytes) {
            1 => DnsType::A(0, 0, 0, 0),
            5 => DnsType::_Cname,
            0_u16 | 2_u16..=4_u16 | 6_u16..=u16::MAX => DnsType::A(0, 0, 0, 0),
        }
    }

    pub fn serialize_to_length_and_data(&self) -> Vec<u8> {
        let mut s = match self {
            DnsType::A(..) => 4u16,
            DnsType::_Cname => todo!(),
        }
        .to_be_bytes()
        .to_vec();
        s.extend_from_slice(&self.serialize());
        s
    }

    pub fn deserialize(type_bytes: [u8; 2], length_and_data_bytes: &[u8]) -> (&[u8], Self) {
        match Self::from_bytes(type_bytes) {
            DnsType::A(..) => Self::deserialize_a_type(length_and_data_bytes),
            DnsType::_Cname => todo!(),
        }
    }

    fn deserialize_a_type(length_and_data_bytes: &[u8]) -> (&[u8], Self) {
        assert_eq!(
            u16::from_be_bytes(
                length_and_data_bytes[..2]
                    .try_into()
                    .expect("Expected to find at least 2 bytes")
            ),
            4
        );
        (
            &length_and_data_bytes[6..],
            DnsType::A(
                length_and_data_bytes[2],
                length_and_data_bytes[3],
                length_and_data_bytes[4],
                length_and_data_bytes[5],
            ),
        )
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
    fn it_serdes() {
        let t = DnsType::A(8, 8, 8, 8);

        let expected_bytes = [0, 4, 8, 8, 8, 8];
        assert_eq!(t.serialize_to_length_and_data(), expected_bytes);
        assert_eq!(
            DnsType::deserialize(1u16.to_be_bytes(), &expected_bytes).1,
            t
        );
    }

    #[test]
    fn it_returns_correct_type_id() {
        assert_eq!(DnsType::A(8, 8, 8, 8).int_as_bytes(), [0, 1]);
        assert_eq!(DnsType::_Cname.int_as_bytes(), [0, 5]);
    }
}
