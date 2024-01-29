use crate::dns_serde::{DnsDeserialize, DnsSerialize};

#[derive(Debug, PartialEq)]
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
    pub fn create_response(mut query_header: DnsHeader) -> DnsHeader {
        query_header.qr = 1;
        query_header.rcode = if query_header.opcode == 0 { 0 } else { 4 };
        query_header
    }
}

impl DnsSerialize for DnsHeader {
    fn serialize(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::with_capacity(12);
        v.extend_from_slice(&self.id.to_be_bytes());

        // next byte includes qr, opcode, aa, tc, rd
        v.push(self.qr << 7 | self.opcode << 3 | self.aa << 2 | self.tc << 1 | self.rd);

        // next byte includes ra, z, rcode
        v.push(self.ra << 7 | self.z << 4 | self.rcode);

        v.extend_from_slice(&self.qdcount.to_be_bytes());
        v.extend_from_slice(&self.ancount.to_be_bytes());
        v.extend_from_slice(&self.nscount.to_be_bytes());
        v.extend_from_slice(&self.arcount.to_be_bytes());
        v
    }
}

impl DnsDeserialize for DnsHeader {
    fn deserialize(data: &[u8]) -> (&[u8], Self) {
        let h = Self {
            id: u16::from_be_bytes(data[..=1].try_into().expect("should have 2 bytes")),

            qr: data[2] >> 7 & 0x01,           //1
            opcode: data[2] >> 3 & 0b00001111, // 4
            aa: data[2] >> 2 & 0x01,           //1
            tc: data[2] >> 1 & 0x01,           //1
            rd: data[2] & 0x01,                //1

            ra: data[3] >> 7 & 0x01,      // 1
            z: data[3] >> 4 & 0b00000111, // 3
            rcode: data[3] & 0b00001111,  // 4

            qdcount: u16::from_be_bytes(data[4..=5].try_into().expect("bytes should exist")),
            ancount: u16::from_be_bytes(data[6..=7].try_into().expect("bytes should exist")),
            nscount: u16::from_be_bytes(data[8..=9].try_into().expect("bytes should exist")),
            arcount: u16::from_be_bytes(data[10..=11].try_into().expect("bytes should exist")),
        };
        (&data[12..], h)
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
    fn it_serdes() {
        let h = DnsHeader {
            id: 1234,
            qr: 1,
            opcode: 2,
            aa: 1,
            tc: 0,
            rd: 1,
            ra: 0,     // 0b00000000
            z: 7,      // 0b00000111
            rcode: 15, // 0b00001111
            qdcount: 2,
            ancount: 2,
            nscount: 7,
            arcount: 8,
        };
        let expected_bytes = [4, 210, 149, 127, 0, 2, 0, 2, 0, 7, 0, 8];
        assert_eq!(h.serialize(), expected_bytes);
        let (remainder, dh) = DnsHeader::deserialize(&expected_bytes);
        assert_eq!(dh, h);
        assert_eq!(remainder.len(), 0);
    }
}
