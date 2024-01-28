use crate::{dns_answer::DnsAnswer, dns_header::DnsHeader, dns_question::DnsQuestion};

pub struct DnsPacket {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsAnswer>,
}

impl DnsPacket {
    pub fn new(
        packet_id: u16,
        query_response_indicator: u8,
        question_names: &[&str],
        answers: Vec<DnsAnswer>,
    ) -> Self {
        Self {
            header: DnsHeader::new(
                packet_id,
                query_response_indicator,
                question_names
                    .len()
                    .try_into()
                    .expect("questions length should fit in 2 bytes"),
                answers
                    .len()
                    .try_into()
                    .expect("answers length should fit in 2 bytes"),
            ),
            questions: question_names
                .into_iter()
                .map(|name| DnsQuestion::new(name))
                .collect(),
            answers,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use crate::dns_type::DnsType;

    use super::*;

    #[test]
    fn it_serializes() {
        let p = DnsPacket::new(
            1234,
            1,
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
