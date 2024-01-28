pub struct DnsQuestion {
    pub name: String,
    pub _type: u16,
    pub _class: u16,
}

impl DnsQuestion {
    pub fn new(name: &str) -> Self {
        let mut dq = Self::default();
        dq.name = name.into();
        dq
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();

        // serialize the name
        self.name
            .split(".")
            .into_iter()
            .map(|label| label.as_bytes())
            .for_each(|label_bytes| {
                v.push(
                    label_bytes
                        .len()
                        .try_into()
                        .expect("label length should not be longer than 128 bytes"),
                );
                v.extend_from_slice(&label_bytes)
            });

        v.push(0x0);

        v.extend_from_slice(&self._type.to_be_bytes());
        v.extend_from_slice(&self._class.to_be_bytes());
        v
    }
}

impl Default for DnsQuestion {
    fn default() -> Self {
        Self {
            name: "".into(),
            _type: 0,
            _class: 0,
        }
    }
}
