use sha1::Digest;

pub trait Hasher: Sized {
    fn new() -> Self;
    fn update(&mut self, input: &[u8]);
    fn finalize(self) -> Vec<u8>;

    fn hash(input: &[u8]) -> Vec<u8> {
        let mut hasher = Self::new();
        hasher.update(input);
        hasher.finalize()
    }
}

struct Sha1(::sha1::Sha1);

impl Hasher for Sha1 {
    fn new() -> Self {
        Self(<::sha1::Sha1 as ::sha1::Digest>::new())
    }
    fn update(&mut self, input: &[u8]) {
        self.0.update(input);
    }
    fn finalize(self) -> Vec<u8> {
        self.0.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {

    use hex_literal::hex;
    use super::Hasher;

    #[test]
    fn sha1_hash() {
        let raw_data = r#"{"nickName":"Band","gender":1,"language":"zh_CN","city":"Guangzhou","province":"Guangdong","country":"CN","avatarUrl":"http://wx.qlogo.cn/mmopen/vi_32/1vZvI39NWFQ9XM4LtQpFrQJ1xlgZxx3w7bQxKARol6503Iuswjjn6nIGBiaycAjAtpujxyzYsrztuuICqIM5ibXQ/0"}"#;
        let key = "HyVFkGl5F5OQWJZZaNzBBg==";
        let input: Vec<u8> = (raw_data.to_owned() + key).into();
        let result = crate::hash::Sha1::hash(&input);
        assert_eq!(result[..], hex!("75e81ceda165f4ffa64f4068af58c64b8f54b88c"));
    }
}
