use crate::dns_serde::{DnsDeserialize, DnsSerialize};
use crate::dns_type::DnsType;
use crate::label_seq::LabelSeq;

#[derive(Debug, PartialEq, Clone)]
pub struct DnsQuestion {
    pub name: LabelSeq,
    pub _type: DnsType,
    pub _class: u16,
}

impl DnsSerialize for DnsQuestion {
    fn serialize(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        // serialize the name
        v.extend_from_slice(&self.name.serialize());
        v.extend_from_slice(&self._type.int_as_bytes());
        v.extend_from_slice(&self._class.to_be_bytes());
        v
    }
}

impl DnsDeserialize for DnsQuestion {
    fn deserialize(data: &[u8]) -> (&[u8], Self) {
        let (remainder, name) = LabelSeq::deserialize(data);
        let _type = DnsType::from_bytes(remainder[..2].try_into().expect("Invalid data"));
        let _class = u16::from_be_bytes(remainder[2..4].try_into().expect("Invalid data"));
        (
            &remainder[4..],
            Self {
                name,
                _type,
                _class,
            },
        )
    }
}

impl Default for DnsQuestion {
    fn default() -> Self {
        Self {
            name: LabelSeq::default(),
            _type: DnsType::default(),
            _class: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serdes() {
        let q = DnsQuestion {
            name: LabelSeq::_new("codecrafters.io"),
            ..Default::default()
        };
        let expected_bytes = [
            12, 99, 111, 100, 101, 99, 114, 97, 102, 116, 101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1,
        ];
        assert_eq!(q.serialize(), expected_bytes);
        let (remainder, dq) = DnsQuestion::deserialize(&expected_bytes);
        assert_eq!(remainder.len(), 0);
        assert_eq!(dq, q);
    }
}
