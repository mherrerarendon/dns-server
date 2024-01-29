use crate::{
    dns_answer::DnsAnswer,
    dns_header::DnsHeader,
    dns_question::DnsQuestion,
    dns_serde::{DnsDeserialize, DnsSerialize},
};

pub struct DnsPacket {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsAnswer>,
}

impl DnsPacket {
    pub fn new(mut header: DnsHeader, question_names: &[&str], answers: Vec<DnsAnswer>) -> Self {
        header.qdcount = question_names
            .len()
            .try_into()
            .expect("questions length should fit in 2 bytes");
        header.ancount = answers
            .len()
            .try_into()
            .expect("answers length should fit in 2 bytes");
        Self {
            header,
            questions: question_names
                .into_iter()
                .map(|name| DnsQuestion::new(name))
                .collect(),
            answers,
        }
    }

    pub fn into_parts(self) -> (DnsHeader, Vec<DnsQuestion>, Vec<DnsAnswer>) {
        (self.header, self.questions, self.answers)
    }
}

impl DnsSerialize for DnsPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut p: Vec<u8> = Vec::new();
        p.extend_from_slice(&self.header.serialize());
        for question in &self.questions {
            p.extend_from_slice(&question.serialize());
        }
        for answer in &self.answers {
            p.extend_from_slice(&answer.serialize());
        }
        p
    }
}

impl DnsDeserialize for DnsPacket {
    fn deserialize(data: &[u8]) -> (&[u8], Self) {
        let (remainder, header) = DnsHeader::deserialize(&data[..=12]);
        (
            &[],
            Self {
                header,
                questions: vec![DnsQuestion::default()],
                answers: vec![DnsAnswer::default()],
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::dns_type::DnsType;

    use super::*;

    #[test]
    fn it_serializes() {
        let mut h = DnsHeader::default();
        h.id = 1234;
        h.qr = 1;
        let p = DnsPacket::new(
            h,
            &["codecrafters.io"],
            vec![DnsAnswer::new("codecrafters.io", DnsType::A(8, 8, 8, 8))],
        );
        assert_eq!(
            p.serialize(),
            [
                4, 210, 128, 0, 0, 1, 0, 1, 0, 0, 0, 0, 12, 99, 111, 100, 101, 99, 114, 97, 102,
                116, 101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1, 12, 99, 111, 100, 101, 99, 114, 97,
                102, 116, 101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 8, 8, 8, 8
            ]
        )
    }
}
