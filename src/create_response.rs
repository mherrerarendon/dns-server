use crate::{
    dns_answer::DnsAnswer,
    dns_header::DnsHeader,
    dns_packet::DnsPacket,
    dns_serde::{DnsDeserialize, DnsSerialize},
};

pub fn create_response(query_bytes: &[u8; 512]) -> Vec<u8> {
    println!("Deserializing query");
    let (header, questions, _) = DnsPacket::deserialize(query_bytes).1.into_parts();
    println!("Got {} questions", questions.len());
    println!("question 1: {:?}", questions[0].name);

    println!("Creating response header");
    let response_header = DnsHeader::create_response(header);

    println!("Creating answers");
    let answers: Vec<DnsAnswer> = questions
        .iter()
        .map(|question| {
            print!("question into");
            let mut answer: DnsAnswer = question.into();
            print!("resolve answer");
            answer.resolve();
            answer
        })
        .collect();

    println!("Creating packet");
    let dns_packet = DnsPacket::new(response_header, questions, answers);

    print!("Serializing packet");
    dns_packet.serialize()
}
