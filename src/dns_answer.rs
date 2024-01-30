use crate::{
    dns_question::DnsQuestion, dns_serde::DnsSerialize, dns_type::DnsType, label_seq::LabelSeq,
};

#[derive(Debug, PartialEq, Clone)]
pub struct DnsAnswer {
    pub name: LabelSeq,
    pub _type: DnsType,
    pub _class: u16,
    pub ttl: u32,
}

impl DnsAnswer {
    pub fn resolve(&mut self) {
        self._type = match self._type {
            DnsType::A(_, _, _, _) => DnsType::A(8, 8, 8, 8),
            DnsType::_Cname => DnsType::_Cname,
        }
    }
}

impl DnsSerialize for DnsAnswer {
    fn serialize(&self) -> Vec<u8> {
        let mut a: Vec<u8> = Vec::new();
        a.extend_from_slice(&self.name.serialize());
        a.extend_from_slice(&self._type.int_as_bytes());
        a.extend_from_slice(&self._class.to_be_bytes());
        a.extend_from_slice(&self.ttl.to_be_bytes());
        a.extend_from_slice(&self._type.len_as_bytes());
        a.extend_from_slice(&self._type.serialize());
        a
    }
}

impl From<&DnsQuestion> for DnsAnswer {
    fn from(value: &DnsQuestion) -> Self {
        Self {
            ttl: 0,
            name: value.name.clone(),
            _type: value._type.clone(),
            _class: value._class,
        }
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
        let a = DnsAnswer {
            name: LabelSeq::_new("codecrafters.io"),
            _type: DnsType::A(8, 8, 8, 8),
            ..Default::default()
        };
        assert_eq!(
            a.serialize(),
            [
                12, 99, 111, 100, 101, 99, 114, 97, 102, 116, 101, 114, 115, 2, 105, 111, 0, 0, 1,
                0, 1, 0, 0, 0, 0, 0, 4, 8, 8, 8, 8
            ]
        )
    }
}
