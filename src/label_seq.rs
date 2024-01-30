use crate::dns_serde::{DnsDeserialize, DnsSerialize};

#[derive(Debug, PartialEq)]
pub struct LabelSeq {
    name: String,
}

impl LabelSeq {
    pub fn _new(name: &str) -> Self {
        Self { name: name.into() }
    }
}

impl DnsSerialize for LabelSeq {
    fn serialize(&self) -> Vec<u8> {
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

/// Returns the number of bytes parsed, and the parsed label
fn parse_label(data: &[u8]) -> (usize, String) {
    println!("parsing label with: {:#?}", data);
    let len = data[0] as usize;
    String::from_utf8(data[1..=len].into())
        .and_then(|mut label| {
            let new_starting_idx = (len as usize) + 1;
            if data[new_starting_idx] != 0x00 {
                label.push_str(".");
                let (bytes_read, next_segment) = parse_label(&data[new_starting_idx..]);
                label.push_str(&next_segment);
                Ok((bytes_read + len + 1, label)) // +1 for the byte that indicates length
            } else {
                Ok((len + 2, label)) // +1 for the byte that indicates length and +1 for the last null byte
            }
        })
        .expect("invalid data for label")
}

impl DnsDeserialize for LabelSeq {
    fn deserialize(data: &[u8]) -> (&[u8], Self) {
        let (bytes_read, label) = parse_label(data);
        (&data[bytes_read..], Self { name: label })
    }
}

impl Default for LabelSeq {
    fn default() -> Self {
        Self { name: "".into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serdes() {
        let l = LabelSeq::_new("codecrafters.io");
        let expected_bytes = [
            12, 99, 111, 100, 101, 99, 114, 97, 102, 116, 101, 114, 115, 2, 105, 111, 0,
        ];
        assert_eq!(l.serialize(), expected_bytes);
        let (remainder, dl) = LabelSeq::deserialize(&expected_bytes);
        assert_eq!(dl, l);
        assert_eq!(remainder.len(), 0)
    }

    #[test]
    fn it_parses_label() {
        let (bytes_read, label) = parse_label(&[
            0x06, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00,
        ]);
        assert_eq!(bytes_read, 12);
        assert_eq!(label, "google.com");
    }

    #[test]
    fn it_parses_label_with_extra_null_bytes() {
        let bytes = [
            12, 99, 111, 100, 101, 99, 114, 97, 102, 116, 101, 114, 115, 2, 105, 111, 0, 0, 1, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let (bytes_read, label) = parse_label(&bytes);
        assert_eq!(bytes_read, 17);
        assert_eq!(label, "codecrafters.io");
    }
}
