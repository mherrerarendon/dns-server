use crate::{
    dns_answer::DnsAnswer,
    dns_header::DnsHeader,
    dns_packet::DnsPacket,
    dns_serde::{DnsDeserialize, DnsSerialize},
};

pub fn create_response(query_bytes: &[u8; 512]) -> Vec<u8> {
    let (header, questions, _) = DnsPacket::deserialize(query_bytes).1.into_parts();
    let response_header = DnsHeader::create_response(header);
    let answers: Vec<DnsAnswer> = questions
        .iter()
        .map(|question| {
            let mut answer: DnsAnswer = question.into();
            answer.resolve();
            answer
        })
        .collect();
    let dns_packet = DnsPacket::new(response_header, questions, answers);
    dns_packet.serialize()
}
