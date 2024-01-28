mod dns_answer;
mod dns_header;
mod dns_question;
mod dns_type;
mod label_seq;

use dns_answer::DnsAnswer;
use dns_header::DnsHeader;
use dns_question::DnsQuestion;
use dns_type::DnsType;
use std::net::UdpSocket;

struct DnsPacket {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsAnswer>,
}

impl DnsPacket {
    fn new(
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

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let dns_answer = DnsAnswer::new(&"codecrafters.io", DnsType::A(8, 8, 8, 8));
                let dns_packet = DnsPacket::new(1234, 1, &["codecrafters.io"], vec![dns_answer]);
                let response = dns_packet.serialize();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
