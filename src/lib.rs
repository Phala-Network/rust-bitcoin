// Rust Bitcoin Library
// Written in 2014 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Rust Bitcoin Library
//!
//! This is a library for which supports the Bitcoin network protocol and associated
//! primitives. It is designed for Rust programs built to work with the Bitcoin
//! network.
//!
//! It is also written entirely in Rust to illustrate the benefits of strong type
//! safety, including ownership and lifetime, for financial and/or cryptographic
//! software.
//!

// Experimental features we need
#![cfg_attr(all(test, feature = "unstable"), feature(test))]

// Coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

#![cfg_attr(all(feature = "mesalock_sgx", not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

// Re-exported dependencies.
#[macro_use] pub extern crate bitcoin_hashes as hashes;
pub extern crate secp256k1;
pub extern crate bech32;
#[cfg(feature = "base64")] pub extern crate base64;

#[cfg(feature="bitcoinconsensus")] extern crate bitcoinconsensus;
#[cfg(feature = "serde")] #[macro_use] extern crate serde;
#[cfg(all(test, feature = "serde"))] extern crate serde_json;
#[cfg(all(test, feature = "serde"))] extern crate serde_test;
#[cfg(all(test, feature = "serde"))] extern crate bincode;
#[cfg(all(test, feature = "unstable"))] extern crate test;

#[cfg(target_pointer_width = "16")]
compile_error!("rust-bitcoin cannot be used on 16-bit architectures");

#[cfg(test)]
#[macro_use]
mod test_macros;
#[macro_use]
mod internal_macros;
#[cfg(feature = "serde")]
mod serde_utils;

#[macro_use]
pub mod network;
pub mod blockdata;
pub mod util;
pub mod consensus;
pub mod hash_types;

pub use crate::hash_types::*;
pub use crate::blockdata::block::Block;
pub use crate::blockdata::block::BlockHeader;
pub use crate::blockdata::script::Script;
pub use crate::blockdata::transaction::Transaction;
pub use crate::blockdata::transaction::TxIn;
pub use crate::blockdata::transaction::TxOut;
pub use crate::blockdata::transaction::OutPoint;
pub use crate::blockdata::transaction::SigHashType;
pub use crate::consensus::encode::VarInt;
pub use crate::network::constants::Network;
pub use crate::util::Error;
pub use crate::util::address::Address;
pub use crate::util::address::AddressType;
pub use crate::util::amount::Amount;
pub use crate::util::amount::Denomination;
pub use crate::util::amount::SignedAmount;
pub use crate::util::key::PrivateKey;
pub use crate::util::key::PublicKey;
pub use crate::util::merkleblock::MerkleBlock;

#[cfg(all(test, feature = "unstable"))] use tests::EmptyWrite;

#[cfg(all(test, feature = "unstable"))]
mod tests {
    use hashes::core::fmt::Arguments;
    use std::io::{IoSlice, Result, Write};

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
    pub struct EmptyWrite;

    impl Write for EmptyWrite {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn write_vectored(&mut self, bufs: &[IoSlice]) -> Result<usize> {
            Ok(bufs.iter().map(|s| s.len()).sum())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }

        fn write_all(&mut self, _: &[u8]) -> Result<()> {
            Ok(())
        }
        fn write_fmt(&mut self, _: Arguments) -> Result<()> {
            Ok(())
        }
    }
}
