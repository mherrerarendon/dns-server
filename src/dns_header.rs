pub struct DnsHeader {
    pub id: u16,
    pub qr: u8,     // 1 bit
    pub opcode: u8, // 4 bits
    pub aa: u8,     // 1 bit
    pub tc: u8,     // 1 bit
    pub rd: u8,     // 1 bit
    pub ra: u8,     // 1 bit
    pub z: u8,      // 3 bits
    pub rcode: u8,  // 4 bits
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DnsHeader {
    pub fn new(
        packet_id: u16,
        query_response_indicator: u8,
        question_count: u16,
        answer_count: u16,
    ) -> Self {
        let mut h = Self::default();
        h.id = packet_id;
        h.qr = query_response_indicator;
        h.qdcount = question_count;
        h.ancount = answer_count;
        h
    }

    pub fn serialize(&self) -> Vec<u8> {
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

impl Default for DnsHeader {
    fn default() -> Self {
        Self {
            id: 0,
            qr: 0,
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes() {
        let h = DnsHeader::new(1234, 1, 2, 2);
        assert_eq!(h.serialize(), [4, 210, 128, 0, 0, 2, 0, 2, 0, 0, 0, 0])
    }
}
