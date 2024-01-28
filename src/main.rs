use std::net::UdpSocket;

struct DnsHeader {
    id: u16,
    qr: u8,     // 1 bit
    opcode: u8, // 4 bits
    aa: u8,     // 1 bit
    tc: u8,     // 1 bit
    rd: u8,     // 1 bit
    ra: u8,     // 1 bit
    z: u8,      // 3 bits
    rcode: u8,  // 4 bits
    qdcount: u16,
    ancount: u16,
    nscount: u16,
    arcount: u16,
}

impl DnsHeader {
    fn serialize(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::with_capacity(12);
        v.extend_from_slice(&self.id.to_be_bytes());

        // next byte includes qr, opcode, aa, tc, rd
        v.push(self.qr << 7 | self.opcode << 6 | self.aa << 2 | self.tc << 1 | self.rd);

        // next byte includes ra, z, rcode
        v.push(self.ra << 7 | self.z << 6 | self.rcode << 3);

        v.extend_from_slice(&self.qdcount.to_be_bytes());
        v.extend_from_slice(&self.ancount.to_be_bytes());
        v.extend_from_slice(&self.nscount.to_be_bytes());
        v.extend_from_slice(&self.arcount.to_be_bytes());
        v
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
                let dns_header = DnsHeader {
                    id: 1234,
                    qr: 1,
                    opcode: 0,
                    aa: 0,
                    tc: 0,
                    rd: 0,
                    ra: 0,
                    z: 0,
                    rcode: 0,
                    qdcount: 0,
                    ancount: 0,
                    nscount: 0,
                    arcount: 0,
                };
                let response = dns_header.serialize();
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
