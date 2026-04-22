//! # FNV Hashing
//!
//! An implementation of the [Fowler–Noll–Vo (FNV)](https://en.wikipedia.org/wiki/Fowler–Noll–Vo_hash_function)
//! non-cryptographic hash function.
//!
//! This crate provides the 64-bit versions of FNV-0, FNV-1, and FNV-1a.
//! FNV hashes are designed to be fast while maintaining a low collision rate,
//! making them ideal for use in lookup tables and hash maps.

#![no_std]

mod constants;
mod fvn;
mod fvn1a;

pub use constants::*;
pub use fvn::*;
pub use fvn1a::*;

/// An alias for the default 64-bit FNV hasher.
///
/// This currently points to [`Fvn1aHasher`], which is the recommended
/// variant for most applications due to its superior bit-dispersion properties.
pub use fvn1a::Fvn1aHasher as FvnHasher;

/// An alias for the default 64-bit FNV build hasher.
///
/// This provides a convenient way to use FNV with standard library collections:
///
/// ```rust
/// use std::collections::HashMap;
/// use fnv64_rs::FvnBuildHasher;
///
/// let mut map: HashMap<String, i32, FvnBuildHasher> = HashMap::default();
/// map.insert("apple".to_string(), 1);
/// ```
pub use fvn1a::Fvn1aBuildHasher as FvnBuildHasher;
