use crate::{
    dns_answer::DnsAnswer,
    dns_header::DnsHeader,
    dns_packet::DnsPacket,
    dns_question::DnsQuestion,
    dns_serde::{DnsDeserialize, DnsSerialize},
};

fn questions_as_answers(questions: &[DnsQuestion]) -> Vec<DnsAnswer> {
    questions
        .iter()
        .map(|question| {
            let mut answer: DnsAnswer = question.into();
            answer.resolve();
            answer
        })
        .collect()
}
pub fn create_response(query_bytes: &[u8; 512]) -> Vec<u8> {
    let (header, questions, _) = DnsPacket::deserialize(query_bytes).1.into_parts();
    let response_header = DnsHeader::create_response(header);
    let answers = questions_as_answers(&questions);
    let dns_packet = DnsPacket::new(response_header, questions, answers);

    dns_packet.serialize()
}

#[cfg(test)]
mod tests {
    use crate::{dns_type::DnsType, label_seq::LabelSeq};

    use super::*;

    #[test]
    fn it_converts_questions_to_answers() {
        let q = DnsQuestion {
            name: LabelSeq::_new("google.com"),
            ..Default::default()
        };
        let ans = questions_as_answers(&vec![q]);
        assert_eq!(ans.len(), 1);
        assert_eq!(
            ans[0],
            DnsAnswer {
                name: LabelSeq::_new("google.com"),
                _type: DnsType::A(8, 8, 8, 8),
                ..Default::default()
            }
        )
    }
}
