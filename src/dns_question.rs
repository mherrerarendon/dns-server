use crate::dns_type::DnsType;
use crate::label_seq::LabelSeq;

pub struct DnsQuestion {
    pub name: LabelSeq,
    pub _type: DnsType,
    pub _class: u16,
}

impl DnsQuestion {
    pub fn new(name: &str) -> Self {
        let mut dq = Self::default();
        dq.name = LabelSeq::new(name);
        dq
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        // serialize the name
        v.extend_from_slice(&self.name.serialize());
        v.extend_from_slice(&self._type.as_int_bytes());
        v.extend_from_slice(&self._class.to_be_bytes());
        v
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
    fn it_serializes() {
        let q = DnsQuestion::new("codecrafters.io");
        assert_eq!(
            q.serialize(),
            [
                12, 99, 111, 100, 101, 99, 114, 97, 102, 116, 101, 114, 115, 2, 105, 111, 0, 0, 1,
                0, 1
            ]
        )
    }
}
