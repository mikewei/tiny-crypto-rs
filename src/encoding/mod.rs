//! Text Encoders
//! 

/// The trait for encoders to encode to / decode from string.
pub trait Encoder {
    type Error: std::error::Error;

    fn to_text(&self, input: &[u8]) -> String;
    fn from_text(&self, input: &str) -> Result<Vec<u8>, Self::Error>;
}

mod base64;
/// BASE64 is a constant Base64 instance for convenience.
pub const BASE64: base64::Base64 = base64::Base64;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(BASE64)]
    fn to_and_from_text(#[case] encoder: impl Encoder) {
        let plain = "some text".as_bytes();
        assert_eq!(plain, &encoder.from_text(&encoder.to_text(plain)).unwrap());
    }
}