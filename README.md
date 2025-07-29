# Custom Sysvar in Agave Validator
This repository contains a custom sysvar implementation for the Solana blockchain runtime (Agave validator) that just stores the string greeting **"GM GM"**.

## Setup
Same as custom syscall setup you need to clone the [Agave](https://github.com/anza-xyz/agave.git), [solana-sdk](https://github.com/anza-xyz/solana-sdk), [system](https://github.com/solana-program/system), [stake](https://github.com/solana-program/stake)

## Implementation
1. Define syscall in [solana-sdk/define-syscall](https://github.com/Kym0211/solana-sdk/blob/master/define-syscall/src/definitions.rs#L42)
```
define_syscall!(fn sol_get_greet_sysvar(addr: *mut u8) -> u64);
```

2. Implement Greet sysvar in solana-sdk repo

- ``` cargo init greet```
- In src file rename main.rs to lib.rs and make new file sysvar.rs
- In cargo.toml add this
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
- In lib.rs add this 
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
- In sysvar.rs add this
```
pub use solana_sdk_ids::sysvar::greet::{check_id, id, ID};
use {crate::Greet, solana_sysvar_id::impl_sysvar_id};

impl_sysvar_id!(Greet);
```

3. Include sysvar in [solana-sdk/program](https://github.com/Kym0211/solana-sdk/blob/bd0c1f0ad2ecd92542e63e0ce14508cc5d06d61c/program/src/sysvar.rs#L8)
```
solana_sysvar::greet
```

4. Define the id of sysvar in [solana-sdk/sdk-ids](https://github.com/Kym0211/solana-sdk/blob/master/sdk-ids/src/lib.rs#L83-L85)
```
pub mod greet {
    solana_pubkey::declare_id!("SysvarGreet11111111111111111111111111111111");
}
```

5. Add **greet.rs** in [solana-sdk/sysvar](https://github.com/Kym0211/solana-sdk/blob/master/sysvar/src/greet.rs)
```
//! The greet sysvar provides greeting ie "GM GM" 
//!
//! [`Greet`] implements [`Sysvar::get`] and can be loaded efficiently without
//! passing the sysvar account ID to the program.
//!
//! See also the Solana [SIMD proposal][simd].
//!
//! [simd]: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0047-syscall-and-sysvar-for-last-restart-slot.md
//!
//! # Examples
//!
//! Accessing via on-chain program directly:
//!
//! ```no_run
//! # use solana_account_info::AccountInfo;
//! # use solana_msg::msg;
//! # use solana_sysvar::Sysvar;
//! # use solana_program_error::ProgramResult;
//! # use solana_pubkey::Pubkey;
//! # use solana_greet::Greet;
//!
//! fn process_instruction(
//!     program_id: &Pubkey,
//!     accounts: &[AccountInfo],
//!     instruction_data: &[u8],
//! ) -> ProgramResult {
//!
//!     let greet = Greet::get();
//!     msg!("last restart slot: {:?}", greet.greeting);
//!
//!     Ok(())
//! }
//! ```
//!
#[cfg(feature = "bincode")]
use crate::SysvarSerialize;
use crate::{impl_sysvar_get, Sysvar};
pub use {
    solana_greet::Greet,
    solana_sdk_ids::sysvar::greet::{check_id, id, ID},
};

impl Sysvar for Greet {
    impl_sysvar_get!(sol_get_greet_sysvar);
}

// #[cfg(feature = "bincode")]
// impl SysvarSerialize for Greet {}
``` 
Add this line in [cargo.toml](https://github.com/Kym0211/solana-sdk/blob/master/sysvar/Cargo.toml#L57)
```
solana-greet = { workspace = true, features = ["sysvar"] }
```
and also in [lib.rs](https://github.com/Kym0211/solana-sdk/blob/master/sysvar/src/lib.rs#L102)
```
pub mod greet;
```


6. Finally Implement syscall stubs in `solana-sdk/sysvar`
```
fn sol_get_greet_sysvar(&self, _var_addr: *mut u8) -> u64 {
    UNSUPPORTED_SYSVAR
}
```
[Code Link](https://github.com/Kym0211/solana-sdk/blob/master/sysvar/src/program_stubs.rs#L75-L77)
```
pub(crate) fn sol_get_greet_sysvar(var_addr: *mut u8) -> u64 {
    SYSCALL_STUBS.read().unwrap().sol_get_greet_sysvar(var_addr)
}
```
[Code Link](https://github.com/Kym0211/solana-sdk/blob/master/sysvar/src/program_stubs.rs#L190-L192)

## Sample Usage
Now as all the setup is completed, add these dependencies to your solana program.
```
use solana_program::sysvar::greet::Greet;

pub fn process_instruction_with_greet() -> u64 {
    let greet = Greet::default();
    println!("{}", greet.greeting);
    0
}

#[test]
fn test_process_instruction_with_greet() {
    // let greet = Greet { greeting: "Gm Gm".to_string() };
    let output = process_instruction_with_greet();
    assert_eq!(output, 0);
}
```
Run test if its successfull then you are good to go, if not then brainstorm in it what cause the error how can i solve it, or else you can contact me anytime 
**discord** - _kavyam

### Note - This project is just meant for study purpose only dont expect it'll work in mainnet