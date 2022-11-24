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

/// Go encoding library
#[cfg(feature = "encoding")]
#[cfg_attr(docsrs, doc(cfg(feature = "encoding")))]
pub mod encoding;


/// Copies elements from a source slice into a destination slice. (As a special case, it also will copy bytes from a string to a slice of bytes.) The source and destination may overlap. 
/// Copy returns the number of elements copied, which will be the minimum of `src.len()` and `dst.len()`.
#[inline]
pub fn copy<T: core::marker::Copy>(src: &[T], dst: &mut [T]) -> usize {
    let min_len = core::cmp::min(src.len(), dst.len());
    dst[..min_len].copy_from_slice(&src[..min_len]);
    min_len
}