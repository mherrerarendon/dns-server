pub trait DnsSerialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait DnsDeserialize {
    fn deserialize(data: &[u8]) -> Self;
}
