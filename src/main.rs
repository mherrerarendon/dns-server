mod dns_answer;
mod dns_header;
mod dns_packet;
mod dns_question;
mod dns_type;
mod label_seq;

use dns_answer::DnsAnswer;
use dns_packet::DnsPacket;
use dns_type::DnsType;
use std::net::UdpSocket;

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
