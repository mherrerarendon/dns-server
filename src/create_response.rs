use crate::{
    dns_answer::DnsAnswer,
    dns_header::DnsHeader,
    dns_packet::DnsPacket,
    dns_serde::{DnsDeserialize, DnsSerialize},
    dns_type::DnsType,
};

pub fn create_response(query_bytes: &[u8; 512]) -> Vec<u8> {
    let (header, questions, _) = DnsPacket::deserialize(query_bytes).1.into_parts();
    let response_header = DnsHeader::create_response(header);
    let dns_answer = DnsAnswer::new(&"codecrafters.io", DnsType::A(8, 8, 8, 8));
    let dns_packet = DnsPacket::new(response_header, &["codecrafters.io"], vec![dns_answer]);
    dns_packet.serialize()
}
