pub trait Hasher {
    fn new() -> Self;
    fn update(&mut self, input: &[u8]);
    fn finalize(self) -> Vec<u8>;
}

pub trait OnceHasher {
    fn hash(&self, input: &[u8]) -> Vec<u8>;
}

pub struct OnceWrapper<H: Hasher>(PhantomData<H>);

impl<H: Hasher> OnceHasher for OnceWrapper<H> {
    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = H::new();
        hasher.update(input);
        hasher.finalize()
    }
}

mod sha1;
use std::marker::PhantomData;

pub use sha1::Sha1;
pub const SHA1: OnceWrapper<Sha1> = OnceWrapper(PhantomData);

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::*;
    use hex_literal::hex;

    #[rstest]
    #[case(Sha1::new())]
    fn update_finalize(#[case] mut hasher: impl Hasher) {
        // let mut hasher = Sha1::new();
        let data1 = r#"{"nickName":"Band","gender":1,"language":"zh_CN","city":"Guangzhou","province":"Guangdong","country":"CN","avatarUrl":"http://wx.qlogo.cn/mmopen/vi_32/1vZvI39NWFQ9XM4LtQpFrQJ1xlgZxx3w7bQxKARol6503Iuswjjn6nIGBiaycAjAtpujxyzYsrztuuICqIM5ibXQ/0"}"#;
        let data2 = "HyVFkGl5F5OQWJZZaNzBBg==";
        hasher.update(data1.as_bytes());
        hasher.update(data2.as_bytes());
        let result = hasher.finalize();
        assert_eq!(result[..], hex!("75e81ceda165f4ffa64f4068af58c64b8f54b88c"));
    }

    #[rstest]
    #[case(SHA1)]
    fn hash(#[case] hasher: impl OnceHasher) {
        let data = r#"{"nickName":"Band","gender":1,"language":"zh_CN","city":"Guangzhou","province":"Guangdong","country":"CN","avatarUrl":"http://wx.qlogo.cn/mmopen/vi_32/1vZvI39NWFQ9XM4LtQpFrQJ1xlgZxx3w7bQxKARol6503Iuswjjn6nIGBiaycAjAtpujxyzYsrztuuICqIM5ibXQ/0"}HyVFkGl5F5OQWJZZaNzBBg=="#;
        let result = hasher.hash(data.as_bytes());
        assert_eq!(result[..], hex!("75e81ceda165f4ffa64f4068af58c64b8f54b88c"));
    }
}
