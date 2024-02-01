pub trait DnsSerialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait DnsDeserialize: Sized {
    fn deserialize(data: &[u8]) -> (&[u8], Self);

    fn deserialize_multiple(data: &[u8], count: usize) -> (&[u8], Vec<Self>) {
        let mut new_remainder = data;
        let mut items: Vec<Self> = Vec::new();
        for _ in 0..count {
            let (remainder, item) = Self::deserialize(&new_remainder);
            new_remainder = remainder;
            items.push(item);
        }
        (new_remainder, items)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dns_question::DnsQuestion,
        dns_serde::{DnsDeserialize, DnsSerialize},
        label_seq::LabelSeq,
    };

    #[test]
    fn it_serdes_multiple() {
        let q1 = DnsQuestion {
            name: LabelSeq::_new("codecrafters.io"),
            ..Default::default()
        };
        let q2 = DnsQuestion {
            name: LabelSeq::_new("google.com"),
            ..Default::default()
        };
        let mut the_bytes = q1.serialize();
        the_bytes.extend_from_slice(&q2.serialize());

        let (remainder, r) = DnsQuestion::deserialize_multiple(&the_bytes, 2);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0], q1);
        assert_eq!(r[1], q2);
        assert_eq!(remainder.len(), 0);
    }
}
