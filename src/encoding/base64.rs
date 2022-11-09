use std::iter::FromIterator;
use crate::encoding::Encoder;

const BYTE_LENGTH: usize = 3;

const TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub struct Base64;

impl Base64 {

    fn to_base_char(&self, bytes: Vec<u8>, pad_len: usize) -> String {
        let base64_chars: Vec<char> = TABLE.chars().collect();

        let mut base_chars: Vec<char> = vec!();
        let chunks = bytes.chunks(BYTE_LENGTH);
        let count = &chunks.len();
        for (i, c) in chunks.enumerate() {
            let c0 = &c[0];
            let c1 = &c[1];
            let c2 = &c[2];

            let first = (c0 & 252) / 4;
            let second = ((c0 & 3) * 16) + ((c1 & 240) / 16);
            let third = ((c1 & 15) * 4) + ((c2 & 192) / 64);
            let fourth = c2 & 63;

            base_chars.push(base64_chars[first as usize]);
            base_chars.push(base64_chars[second as usize]);
            if pad_len == 2 && *count == i + 1 {
                base_chars.push('=');
            } else {
                base_chars.push(base64_chars[third as usize]);
            }

            if pad_len >= 1 && *count == i + 1 {
                base_chars.push('=');
            } else {
                base_chars.push(base64_chars[fourth as usize]);
            }
        }

        String::from_iter(base_chars.iter())
    }
}

impl Encoder for Base64 {
    fn encode(&self, s: &str) -> String {

        // let bytes = s.bytes();
        let bytes = s.as_bytes();
        // let three_bytes: Chunks<u8> = as_3bytes(bytes);

        let remain = bytes.len() % BYTE_LENGTH;
        let pad_len = match remain {
            1 | 2 => BYTE_LENGTH - remain,
            _ => 0,
        };
        let mut v = bytes.to_vec();
        if pad_len != 0 {
            for _ in 0..pad_len {
                v.push(b'\0')
            }
        }

        self.to_base_char(v, pad_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_encode() {
        let base64 = Base64{};

        assert_eq!("", base64.encode(""));
        assert_eq!("Zg==", base64.encode("f"));
        assert_eq!("Zm8=", base64.encode("fo"));
        assert_eq!("Zm9v", base64.encode("foo"));
        assert_eq!("Zm9vYg==", base64.encode("foob"));
    }

    #[test]
    fn test() {
        // let slice = &["a", "b", "c"];
        // let mut v = slice.to_vec();
        // v.push("d");
        // assert_eq!("d", v[3]);
        // let i = 15;
        // assert_eq!(0, 12 | 3);

        //
        // 64 + 32 + 4
        // 01100100
        // let u = 100u8;
        // // assert_eq!(25, ((u & 128) + (u & 64) + (u & 32) + (u & 16) + (u & 8) + (u & 4)) / 4);
        // assert_eq!(25, (u & 252) / 4);

        //
        // let s = "AIUEO";
        // assert_eq!("A", s.chars().nth(1).unwrap().to_string());
        // let c: Vec<char> = vec!('A', 'B', 'C');
        // assert_eq!("ABC", String::from_iter(c.iter()).as_str());

        //
        let base64_chars: Vec<char> = TABLE.chars().collect();
        assert_eq!('Z', base64_chars[25]);
        assert_eq!('g', base64_chars[32]);
        // println!("{:?}", base64_chars.nth(32u8 as usize).unwrap());

        // assert_eq!('Z', base64_chars.nth(25u8 as usize).unwrap());
        // assert_eq!('g', base64_chars.nth(32u8 as usize).unwrap());
        // assert_eq!(32, 32u8 as usize);
    }
}
