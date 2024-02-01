use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

use crate::{
    dns_packet::DnsPacket,
    dns_serde::{DnsDeserialize, DnsSerialize},
};

pub struct QueryHandler {
    pending_queries: HashMap<u16, (SocketAddr, DnsPacket)>, // <packet_id, (source_addr, packet)>
}

impl QueryHandler {
    pub fn new() -> Self {
        Self {
            pending_queries: HashMap::new(),
        }
    }
    pub fn handle_query(
        &mut self,
        query_bytes: &[u8; 512],
        source_addr: SocketAddr,
        resolver: Option<(&str, &UdpSocket)>,
    ) {
        if let Some((resolver_addr, socket)) = resolver {
            let query_packet = DnsPacket::deserialize(query_bytes).1;
            if query_packet.header.qr == 0 {
                // query is a question
                println!(
                    "handling {} questions from {}",
                    query_packet.header.qdcount, source_addr
                );
                let response_packet = query_packet.clone();
                self.pending_queries
                    .insert(query_packet.header.id, (source_addr, response_packet));
                for question in query_packet.questions {
                    let forward_header = query_packet.header.clone();
                    let forward_packet = DnsPacket::new(forward_header, vec![question], None);
                    let forward_bytes = forward_packet.serialize();
                    println!("forwarding question to {}", resolver_addr);
                    socket
                        .send_to(&forward_bytes, resolver_addr)
                        .expect("Failed to forward query");
                }
            } else {
                // is answer from resolver
                println!("handling answer from {}", source_addr);
                let (header, _, answers) = DnsPacket::deserialize(query_bytes).1.into_parts();

                if let (Some(ref mut pending_query), Some(answers)) =
                    (self.pending_queries.get_mut(&header.id), answers)
                {
                    println!("adding answer to pending query from {}", pending_query.0);
                    pending_query.1.add_answer(answers[1].clone());
                    if pending_query.1.all_questions_answered() {
                        let resolved_bytes = pending_query.1.serialize();
                        socket
                            .send_to(&resolved_bytes, pending_query.0)
                            .expect("Failed to respond to query");
                    }
                }
            }
        }
    }
}
