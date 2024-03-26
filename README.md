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
  - Hex

## Example

Use sha1 hasher:
```rust
let result: Vec<u8> = tiny_crypto::sha1!(b"abcdefghijklmn", b"opqrstuvwxyz");
assert_eq!(result, tiny_crypto::sha1!(b"abcdefghijklmnopqrstuvwxyz"));

let hex_result: String = tiny_crypto::sha1_hex!(b"abcdefgh");
assert_eq!(hex_result, "425af12a0743502b322e93a015bcf868e324d56a");
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
let origin: &[u8] = b"some bytes to encode";
assert_eq!(origin, &BASE64.from_text(&BASE64.to_text(origin)).unwrap());
```

Use Hex encoder:
```rust
use tiny_crypto::encoding::{Encoder, HEX};
let origin: &[u8] = b"some bytes to encode";
assert_eq!(origin, &HEX.from_text(&HEX.to_text(origin)).unwrap());
```


License: MIT
