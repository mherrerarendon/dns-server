use crate::{
    dns_answer::DnsAnswer,
    dns_packet::DnsPacket,
    dns_serde::{DnsDeserialize, DnsSerialize},
    dns_type::DnsType,
};

pub fn create_response(query_bytes: &[u8; 512]) -> Vec<u8> {
    let (mut header, _, _) = DnsPacket::deserialize(query_bytes).into_parts();
    header.qr = 1;
    let dns_answer = DnsAnswer::new(&"codecrafters.io", DnsType::A(8, 8, 8, 8));
    let dns_packet = DnsPacket::new(header, &["codecrafters.io"], vec![dns_answer]);
    dns_packet.serialize()
}
