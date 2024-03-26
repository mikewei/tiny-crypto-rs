//! Cryptographic Hash Functions
//! 

use std::marker::PhantomData;

/// The hash function trait with multiple updates.
pub trait Hasher {
    fn new() -> Self;
    fn update(&mut self, input: &[u8]);
    fn finalize(self) -> Vec<u8>;
}

/// The hash function trait with one-shot hash call.
pub trait OnceHasher {
    type Hasher: Hasher;

    fn hash(&self, input: &[u8]) -> Vec<u8>;
    fn new_hasher(&self) -> Self::Hasher {
        Self::Hasher::new()
    }
}

/// The type wraps a Hahser to a OnceHasher
pub struct OnceWrapper<H: Hasher>(PhantomData<H>);

impl<H: Hasher> OnceHasher for OnceWrapper<H> {
    type Hasher = H;

    fn hash(&self, input: &[u8]) -> Vec<u8> {
        let mut hasher = H::new();
        hasher.update(input);
        hasher.finalize()
    }
}

/// A macro to call hash with variadic arguments.
#[macro_export]
macro_rules! hash {
    ($ohasher:ident, $($arg:expr),+) => {
        {
            use $crate::hash::{Hasher, OnceHasher};
            let mut hasher = $crate::hash::$ohasher.new_hasher();
            $(hasher.update($arg);)+
            hasher.finalize()
        }
    };
}

/// A macro to call hash with variadic arguments and encode to text.
#[macro_export]
macro_rules! hash_and_encode {
    ($ohasher:ident, $encoder:ident, $($arg:expr),+) => {
        {
            use $crate::hash::{Hasher, OnceHasher};
            use $crate::encoding::{Encoder};
            let mut hasher = $crate::hash::$ohasher.new_hasher();
            $(hasher.update($arg);)+
            $crate::encoding::$encoder.to_text(&hasher.finalize())
        }
    };
}

mod sha1;
pub use sha1::Sha1;

/// SHA1 is a constant Sha1 instance with OnceHasher trait.
pub const SHA1: OnceWrapper<Sha1> = OnceWrapper(PhantomData);

/// A macro to call sha1 hash with variadic arguments.
#[macro_export]
macro_rules! sha1 {
    ($($arg:expr),+) => {
        $crate::hash!(SHA1, $($arg),+)
    };
}

/// A macro to call sha1 hash with variadic arguments and encode to hex string.
#[macro_export]
macro_rules! sha1_hex {
    ($($arg:expr),+) => {
        $crate::hash_and_encode!(SHA1, HEX, $($arg),+)
    };
}

/// A macro to call sha1 hash with variadic arguments and encode to base64 string.
#[macro_export]
macro_rules! sha1_base64 {
    ($($arg:expr),+) => {
        $crate::hash_and_encode!(SHA1, BASE64, $($arg),+)
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    use hex_literal::hex;
    use rstest::*;

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

    #[test]
    fn macro_test() {
        let data1 = r#"{"nickName":"Band","gender":1,"language":"zh_CN","city":"Guangzhou","province":"Guangdong","country":"CN","avatarUrl":"http://wx.qlogo.cn/mmopen/vi_32/1vZvI39NWFQ9XM4LtQpFrQJ1xlgZxx3w7bQxKARol6503Iuswjjn6nIGBiaycAjAtpujxyzYsrztuuICqIM5ibXQ/0"}"#;
        let data2 = "HyVFkGl5F5OQWJZZaNzBBg==";
        let result = crate::sha1!(data1.as_bytes(), data2.as_bytes());
        assert_eq!(result[..], hex!("75e81ceda165f4ffa64f4068af58c64b8f54b88c"));
        let result = crate::sha1_hex!(data1.as_bytes(), data2.as_bytes());
        assert_eq!(result, "75e81ceda165f4ffa64f4068af58c64b8f54b88c");
        let result = crate::sha1_base64!(data1.as_bytes(), data2.as_bytes());
        assert_eq!(result, "degc7aFl9P+mT0Bor1jGS49UuIw=");
    }
}
