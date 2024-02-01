use crate::{
    dns_answer::DnsAnswer,
    dns_header::DnsHeader,
    dns_question::DnsQuestion,
    dns_serde::{DnsDeserialize, DnsSerialize},
};

#[derive(Debug, Clone, PartialEq)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Option<Vec<DnsAnswer>>,
}

impl DnsPacket {
    pub fn new(
        mut header: DnsHeader,
        questions: Vec<DnsQuestion>,
        answers: Option<Vec<DnsAnswer>>,
    ) -> Self {
        header.qdcount = questions
            .len()
            .try_into()
            .expect("questions length should fit in 2 bytes");
        header.ancount = match &answers {
            Some(answers) => answers
                .len()
                .try_into()
                .expect("answers length should fit in 2 bytes"),
            None => 0,
        };

        Self {
            header,
            questions,
            answers,
        }
    }

    pub fn into_parts(self) -> (DnsHeader, Vec<DnsQuestion>, Option<Vec<DnsAnswer>>) {
        (self.header, self.questions, self.answers)
    }

    pub fn add_answer(&mut self, answer: DnsAnswer) {
        if let Some(ref mut answers) = self.answers {
            answers.push(answer);
        }
    }

    pub fn all_questions_answered(&self) -> bool {
        match self.answers {
            Some(ref answers) => answers.len() == self.header.qdcount as usize,
            None => self.header.qdcount == 0,
        }
    }

    pub fn prepare_for_response(&mut self, qr: u8) {
        self.header.qr = qr;
        self.header.qdcount = self.questions.len() as u16;
        if let Some(ref answers) = self.answers {
            self.header.ancount = answers.len() as u16;
        } else {
            self.header.ancount = 0;
        }
        self.header.rcode = if self.header.opcode == 0 { 0 } else { 4 }
    }
}

impl DnsSerialize for DnsPacket {
    fn serialize(&self) -> Vec<u8> {
        let mut p: Vec<u8> = Vec::new();
        p.extend_from_slice(&self.header.serialize());
        for question in &self.questions {
            p.extend_from_slice(&question.serialize());
        }
        if let Some(answers) = &self.answers {
            for answer in answers {
                p.extend_from_slice(&answer.serialize());
            }
        }
        p
    }
}

impl DnsDeserialize for DnsPacket {
    fn deserialize(data: &[u8]) -> (&[u8], Self) {
        let (remainder, header) = DnsHeader::deserialize(&data);
        let (remainder, questions) =
            DnsQuestion::deserialize_multiple(remainder, header.qdcount as usize);
        let (remainder, answers) =
            DnsAnswer::deserialize_multiple(remainder, header.ancount as usize);
        (
            remainder,
            Self {
                header,
                questions,
                answers: Some(answers),
            },
        )
    }
}

impl Default for DnsPacket {
    fn default() -> Self {
        Self {
            header: DnsHeader::default(),
            questions: vec![Default::default()],
            answers: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{dns_type::DnsType, label_seq::LabelSeq};

    use super::*;

    #[test]
    fn it_serdes() {
        let h = DnsHeader {
            id: 1234,
            qr: 1,
            ..Default::default()
        };
        let q = DnsQuestion::default();
        let a = DnsAnswer {
            name: LabelSeq::_new("codecrafters.io"),
            _type: DnsType::A(8, 8, 8, 8),
            ..Default::default()
        };
        let p = DnsPacket::new(h, vec![q], Some(vec![a]));
        let expected_bytes = [
            4, 210, 128, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 12, 99, 111, 100, 101, 99,
            114, 97, 102, 116, 101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 8, 8,
            8, 8,
        ];
        assert_eq!(p.serialize(), expected_bytes);
        let (remainder, dp) = DnsPacket::deserialize(&expected_bytes);
        assert_eq!(dp, p);
        assert_eq!(remainder.len(), 0);
    }
}
