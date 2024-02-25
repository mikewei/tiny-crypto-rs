use ::sha1::Digest as _;

#[derive(Clone)]
pub struct Sha1(::sha1::Sha1);

impl super::Hasher for Sha1 {
    fn new() -> Self {
        Self(::sha1::Sha1::new())
    }
    fn update(&mut self, input: &[u8]) {
        self.0.update(input);
    }
    fn finalize(self) -> Vec<u8> {
        self.0.finalize().to_vec()
    }
}
