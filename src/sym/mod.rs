//! Symmetric Ciphers
//! 

/// The trait for symmetric cipher.
pub trait Cipher {
    type KeyBytes;

    fn from_key_array(key: &Self::KeyBytes) -> Self;
    fn encrypt_with_iv(&self, iv: &Self::KeyBytes, input: &[u8]) -> Vec<u8>;
    fn decrypt_with_iv(&self, iv: &Self::KeyBytes, input: &[u8]) -> Vec<u8>;
}

mod aes;
pub use aes::Aes;
/// The type of AES with 128 key bits.
pub type Aes128 = aes::Aes<{128 / 8}>;
/// The type of AES with 192 key bits.
pub type Aes192 = aes::Aes<{192 / 8}>;
/// The type of AES with 256 key bits.
pub type Aes256 = aes::Aes<{256 / 8}>;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(Aes128::from_key_array(b"This is the key!"))]
    #[case(Aes192::from_key_array(b"This is the key!This is "))]
    #[case(Aes256::from_key_array(b"This is the key!This is the key!"))]
    fn encrypt_and_decrypt<const N: usize, T: Cipher<KeyBytes=[u8; N]>>(#[case] cipher: T) {
        let plain = b"This is the plain text";
        let iv = [0x88u8; N];
        let data = cipher.encrypt_with_iv(&iv, plain);
        let out = cipher.decrypt_with_iv(&iv, &data);
        assert_eq!(out, plain);
    }
}