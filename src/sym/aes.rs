/// The AES cipher
pub struct Aes<const BYTES: usize> {
    cipher: libaes::Cipher,
}

impl<const BYTES: usize> super::Cipher for Aes<BYTES> {
    type KeyBytes = [u8; BYTES];

    fn from_key_array(key: &Self::KeyBytes) -> Self {
        Self { cipher: new_libaes_cipher(BYTES * 8, key) }
    }

    fn encrypt_with_iv(&self, iv: &Self::KeyBytes, input: &[u8]) -> Vec<u8> {
        self.cipher.cbc_encrypt(iv, input)
    }
    fn decrypt_with_iv(&self, iv: &Self::KeyBytes, input: &[u8]) -> Vec<u8> {
        self.cipher.cbc_decrypt(iv, input)
    }
}

fn new_libaes_cipher(bits: usize, key: &[u8]) -> libaes::Cipher {
    match bits {
        128 => libaes::Cipher::new_128(key.try_into().unwrap()),
        192 => libaes::Cipher::new_192(key.try_into().unwrap()),
        256 => libaes::Cipher::new_256(key.try_into().unwrap()),
        _ => unreachable!()
    }
}