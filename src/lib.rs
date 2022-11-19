//! Porting Go standard library in Rust
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/al8n/crabmole/main/art/crabmole.jpg"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]

#[cfg(feature = "alloc")]
extern crate alloc;

/// Go sort library
#[cfg(feature = "sort")]
#[cfg_attr(docsrs, doc(cfg(feature = "sort")))]
pub mod sort;
