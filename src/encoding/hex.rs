/// The Hex encoder
pub struct Hex;

impl super::Encoder for Hex {
    type Error = hex::FromHexError;

    fn to_text(&self, input: &[u8]) -> String {
        hex::encode(input)
    }

    fn from_text(&self, input: &str) -> Result<Vec<u8>, Self::Error> {
        hex::decode(input)
    }
}