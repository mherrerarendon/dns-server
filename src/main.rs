mod dns_answer;
mod dns_header;
mod dns_packet;
mod dns_question;
mod dns_serde;
mod dns_type;
mod label_seq;
mod query_handler;

use query_handler::QueryHandler;
use std::{env, net::UdpSocket};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let args: Vec<String> = env::args().collect();
    let resolver_addr = &args[2];
    println!("resolver address: {}", resolver_addr);

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    let mut query_handler = QueryHandler::new();

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                query_handler.handle_query(&buf, source, Some((&resolver_addr, &udp_socket)));
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
