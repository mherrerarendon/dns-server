pub enum DnsType {
    A(u8, u8, u8, u8),
    _Cname,
}

impl DnsType {
    pub fn as_int_bytes(&self) -> [u8; 2] {
        match self {
            DnsType::A(_, _, _, _) => 1u16,
            DnsType::_Cname => 5u16,
        }
        .to_be_bytes()
    }

    pub fn len_as_bytes(&self) -> [u8; 2] {
        match self {
            DnsType::A(_, _, _, _) => 4u16,
            DnsType::_Cname => todo!(),
        }
        .to_be_bytes()
    }

    pub fn serialize(&self) -> Vec<u8> {
        match self {
            DnsType::A(a, b, c, d) => vec![*a, *b, *c, *d],
            DnsType::_Cname => todo!(),
        }
    }
}

impl Default for DnsType {
    fn default() -> Self {
        DnsType::A(0, 0, 0, 0)
    }
}
