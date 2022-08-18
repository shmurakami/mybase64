struct Base {
}

// base64
const BYTE_LENGTH: usize = 3;

const BASE64_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

impl Base {
    fn base64_encode(&self, s: &str) -> String {
        /*
        s to byte array
        divide to 3bytes
        pad 0 to right
        convert to 6bits
        to 4 chars with base64 table
         */

        // fn as_3bytes(b: &[u8]) -> Chunks<u8> {
        //     let len = 3;
        //
        //     let remain = b.len() % len;
        //     let mut v = b.to_vec();
        //     if remain != 0 {
        //         let pad_len = len - remain;
        //         for _ in 0..pad_len {
        //             v.push(b'0')
        //         }
        //     }
        //
        //     v.chunks(len)
        // }

        fn to_base_char(bytes: Vec<u8>) -> String {
            let base64_chars: Vec<char> = BASE64_TABLE.chars().collect();

            let mut base_chars: Vec<char> = vec!();
            for chunks in bytes.chunks(BYTE_LENGTH) {
                let c0 = &chunks[0];
                let c1 = &chunks[1];
                let c2 = &chunks[2];

                let first = (c0 & 252) / 4;
                let second = ((c0 & 3) * 16) + ((c1 & 240) / 16);
                let third = ((c1 & 15) * 4) + ((c2 & 192) / 64);
                let fourth = c2 & 63;

                base_chars.push(base64_chars[first as usize]);
                base_chars.push(base64_chars[second as usize]);
                base_chars.push(base64_chars[third as usize]);
                base_chars.push(base64_chars[fourth as usize]);
            }

            String::from_iter(base_chars.iter())
        }

        // let bytes = s.bytes();
        let bytes = s.as_bytes();
        // let three_bytes: Chunks<u8> = as_3bytes(bytes);

        let remain = bytes.len() % BYTE_LENGTH;
        let mut v = bytes.to_vec();
        if remain != 0 {
            let pad_len = BYTE_LENGTH - remain;
            for _ in 0..pad_len {
                v.push(b'\0')
            }
        }

        to_base_char(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::Base;

    #[test]
    fn base64_encode() {
        let base = Base {};

        assert_eq!("", base.base64_encode(""));
        assert_eq!("Zg==", base.base64_encode("f"));
        // assert_eq!("Zm8=", base.base64_encode("fo"));
        // assert_eq!("Zm9v", base.base64_encode("foo"));
        // assert_eq!("Zm9vYg==", base.base64_encode("foob"));
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
        const BASE64_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let base64_chars: Vec<char> = BASE64_TABLE.chars().collect();
        assert_eq!('Z', base64_chars[25]);
        assert_eq!('g', base64_chars[32]);
        // println!("{:?}", base64_chars.nth(32u8 as usize).unwrap());

        // assert_eq!('Z', base64_chars.nth(25u8 as usize).unwrap());
        // assert_eq!('g', base64_chars.nth(32u8 as usize).unwrap());
        // assert_eq!(32, 32u8 as usize);
    }
}