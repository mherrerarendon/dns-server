use crate::{
    dns_answer::DnsAnswer,
    dns_header::DnsHeader,
    dns_question::DnsQuestion,
    dns_serde::{DnsDeserialize, DnsSerialize},
};

#[derive(Debug, Clone)]
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
        let mut questions_remainder = remainder;
        let mut questions: Vec<DnsQuestion> = Vec::new();
        for _ in 0..header.qdcount {
            let (remainder, question) = DnsQuestion::deserialize(&questions_remainder);
            questions_remainder = remainder;
            questions.push(question);
        }
        (
            &[],
            Self {
                header,
                questions,
                answers: None,
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
    fn it_serializes() {
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
        assert_eq!(
            p.serialize(),
            [
                4, 210, 128, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 12, 99, 111, 100, 101,
                99, 114, 97, 102, 116, 101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4,
                8, 8, 8, 8
            ]
        )
    }

    #[test]
    fn it_kinda_serdes() {
        let h = DnsHeader {
            id: 1234,
            qr: 1,
            qdcount: 1,
            ..Default::default()
        };
        let q = DnsQuestion {
            name: LabelSeq::_new("codecrafters.io"),
            ..Default::default()
        };
        let p = DnsPacket {
            header: h.clone(),
            questions: vec![q.clone()],
            ..Default::default()
        };
        let s = p.serialize();
        let (dh, dq, _) = DnsPacket::deserialize(&s).1.into_parts();
        assert_eq!(dh, h);
        assert_eq!(dq[0], q);
    }
}
