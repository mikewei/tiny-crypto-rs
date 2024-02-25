
use base64ct::Encoding as _;

pub struct Base64;

impl super::Encoder for Base64 {
    type Error = base64ct::Error;

    fn to_text(&self, input: &[u8]) -> String {
        base64ct::Base64::encode_string(input)
    }

    fn from_text(&self, input: &str) -> Result<Vec<u8>, Self::Error> {
        base64ct::Base64::decode_vec(input)
    }
}

