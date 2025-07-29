# Custom Sysvar in Agave Validator
This repository contains a custom sysvar implementation for the Solana blockchain runtime (Agave validator) that just stores the string greeting **"GM GM"**.

## Setup
Same as custom syscall setup you need to clone the [Agave](https://github.com/anza-xyz/agave.git), [solana-sdk](https://github.com/anza-xyz/solana-sdk), [system](https://github.com/solana-program/system), [stake](https://github.com/solana-program/stake)

## Implementation
1. Define syscall in [solana-sdk/define-syscall]()
```
define_syscall!(fn sol_get_greet_sysvar(addr: *mut u8) -> u64);
```

2. Implement Greet sysvar in solana-sdk repo

2.1 ``` cargo init greet```
2.2 In src file rename main.rs to lib.rs and make new file sysvar.rs
2.3 In cargo.toml add this
```
[package]
name = "solana-greet"
description = "Types and utilities for the Solana Greet sysvar."
# documentation = "https://docs.rs/solana-last-restart-slot"
version = "0.1.0"
authors = { workspace = true }
repository = { workspace = true }
homepage = { workspace = true }
license = { workspace = true }
edition = { workspace = true }

[dependencies]
serde = { workspace = true, optional = true }
serde_derive = { workspace = true, optional = true }
solana-sdk-ids = { workspace = true, optional = true }
solana-sdk-macro = { workspace = true }
solana-sysvar-id = { workspace = true, optional = true }

[features]
serde = ["dep:serde", "dep:serde_derive"]
sysvar = ["dep:solana-sdk-ids", "dep:solana-sysvar-id"]

[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]
all-features = true
rustdoc-args = ["--cfg=docsrs"]
```
2.4 In lib.rs add this 
```
//! Information about the last restart slot (hard fork).
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "sysvar")]
pub mod sysvar;

#[repr(C)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
#[derive(Debug, PartialEq, Eq)]
pub struct Greet {
    pub greeting: String,
}

impl Default for Greet {
    fn default() -> Self {
        Self {
            greeting: "Gm Gm".to_string(),
        }
    }
}
```
2.5 In sysvar.rs add this
```
pub use solana_sdk_ids::sysvar::greet::{check_id, id, ID};
use {crate::Greet, solana_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Greet);
```