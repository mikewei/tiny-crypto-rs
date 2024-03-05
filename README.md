# tiny-crypto

The tiny-crypto crate is a collection of tools for common crypto algorithms,
with APIs aimed to be simple to use.

## Table of tools
* Cryptographic Hash Functions (hash)
  - Sha1
* Symmetric Ciphers (sym)
  - Aes128, Aes192, Aes256
* Text Encoders (encoding)
  - Base64

## Example

Use sha1 hasher:
```rust
let result = tiny_crypto::sha1!(b"abcdefghijklmn", b"opqrstuvwxyz");
```

Use Aes128 cipher:
```rust
use tiny_crypto::sym::{Cipher, Aes128};
let cipher = Aes128::from_key_array(b"This is the key!");
let plain = b"This is the plain text";
let iv = [0x66u8; 16];
let data = cipher.encrypt_with_iv(&iv, plain);
let out = cipher.decrypt_with_iv(&iv, &data);
assert_eq!(out, plain);
```

Use Base64 encoder:
```rust
use tiny_crypto::encoding::{Encoder, BASE64};
let origin: &[u8] = b"some bytes to encoding";
assert_eq!(origin, &BASE64.from_text(&BASE64.to_text(origin)).unwrap());
```


License: MIT
