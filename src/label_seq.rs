pub struct LabelSeq {
    name: String,
}

impl LabelSeq {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut v: Vec<u8> = Vec::new();
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
        v
    }
}

impl Default for LabelSeq {
    fn default() -> Self {
        Self { name: "".into() }
    }
}
