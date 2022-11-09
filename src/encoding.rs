pub mod base64;

pub trait Encoder {
    fn encode(&self, s: &str) -> String;
}
