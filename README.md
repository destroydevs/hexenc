# HEX
Simple HEX algorithm impl.

Usage: 
```Rust
use hex::Hex;

mod hex;

let hex = Hex::encode("My little string...");
println!("Encoded: {}", &hex);

let str = Hex::decode(hex);
println!("Decoded: {}", str);

assert_eq!(hex, "4d79206c6974746c6520737472696e672e2e2e");
assert_eq!(str, "My little string...");
