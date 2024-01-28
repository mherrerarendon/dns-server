use crate::{dns_type::DnsType, label_seq::LabelSeq};

pub struct DnsAnswer {
    pub name: LabelSeq,
    pub _type: DnsType,
    pub _class: u16,
    pub ttl: u32,
}

impl DnsAnswer {
    pub fn new(name: &str, data: DnsType) -> Self {
        let mut a = Self::default();
        a.name = LabelSeq::new(name);
        a._type = data;
        a
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut a: Vec<u8> = Vec::new();
        a.extend_from_slice(&self.name.serialize());
        a.extend_from_slice(&self._type.as_int_bytes());
        a.extend_from_slice(&self._class.to_be_bytes());
        a.extend_from_slice(&self.ttl.to_be_bytes());
        a.extend_from_slice(&self._type.len_as_bytes());
        a.extend_from_slice(&self._type.serialize());
        a
    }
}

impl Default for DnsAnswer {
    fn default() -> Self {
        Self {
            name: LabelSeq::default(),
            _type: DnsType::default(),
            _class: 1,
            ttl: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes() {
        let a = DnsAnswer::new("codecrafters.io", DnsType::A(8, 8, 8, 8));
        assert_eq!(
            a.serialize(),
            [
                12, 99, 111, 100, 101, 99, 114, 97, 102, 116, 101, 114, 115, 2, 105, 111, 0, 0, 1,
                0, 1, 0, 0, 0, 0, 0, 4, 8, 8, 8, 8
            ]
        )
    }
}
