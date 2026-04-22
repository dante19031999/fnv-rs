//! 🦀 FNV-1 Hashing Implementation
//!
//! This module provides the **FNV-1** variant of the Fowler–Noll–Vo hash function.
//!
//! **Note: All types in this module are re-exported at the crate root.** You can use them
//! via `fnv64_rs::*` instead of `fnv64_rs::fvn1::*`.
//!
//! ### 💡 Algorithm Logic
//! The FNV-1 algorithm follows a **Multiply-then-XOR** sequence:
//! 1. Start with the [`OFFSET_BASIS`][`crate::OFFSET`].
//! 2. For each byte:
//!    * Multiply the current hash by the [`FNV_PRIME`][`crate::PRIME`].
//!    * XOR the result with the byte.
//!
//! ### 🛠️ Usage
//! This module provides both the raw [`Fvn1Hasher`] and the [`Fvn1BuildHasher`] for use with collections.
//!
//! ```rust
//! use core::hash::Hasher;
//! use fnv64_rs::Fvn1Hasher;
//!
//! let mut hasher = Fvn1Hasher::default();
//! hasher.write(b"Rust");
//! let result = hasher.finish();
//! ```

use core::hash::{BuildHasher, Hasher};

/// A generic implementation of the FNV-1 (Fowler–Noll–Vo) hashing algorithm.
///
/// This struct uses const generics for the `OFFSET` and `PRIME` to allow
/// support for different bit-widths or custom variants while maintaining
/// zero-cost abstractions.
pub struct GenericFvn1Hasher<const OFFSET: u64, const PRIME: u64> {
    hash: u64,
}

impl<const OFFSET: u64, const PRIME: u64> Hasher for GenericFvn1Hasher<OFFSET, PRIME> {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    /// Performs the FNV-1 hash transformation: `hash = (hash * PRIME) ^ byte`.
    ///
    /// Note: This uses wrapping multiplication to prevent panics in debug builds,
    /// as the FNV algorithm relies on integer overflow behavior.
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            // FNV-1: Multiply then XOR
            self.hash = self.hash.wrapping_mul(PRIME);
            self.hash ^= *byte as u64;
        }
    }
}

impl<const OFFSET: u64, const PRIME: u64> Default for GenericFvn1Hasher<OFFSET, PRIME> {
    fn default() -> Self {
        Self { hash: OFFSET }
    }
}

/// A builder for creating [`GenericFvn1Hasher`] instances with specific parameters.
pub struct GenericFvn1BuildHasher<const OFFSET: u64, const PRIME: u64> {}

impl<const OFFSET: u64, const PRIME: u64> BuildHasher for GenericFvn1BuildHasher<OFFSET, PRIME> {
    type Hasher = GenericFvn1Hasher<OFFSET, PRIME>;

    fn build_hasher(&self) -> Self::Hasher {
        GenericFvn1Hasher::default()
    }
}

impl<const OFFSET: u64, const PRIME: u64> Default for GenericFvn1BuildHasher<OFFSET, PRIME> {
    fn default() -> Self {
        Self {}
    }
}

/// Standard 64-bit FNV-1 hasher using the official FNV offset basis and prime.
pub type Fvn1Hasher = GenericFvn1Hasher<{ crate::OFFSET }, { crate::PRIME }>;

/// Standard 64-bit FNV-1 build hasher.
pub type Fvn1BuildHasher = GenericFvn1BuildHasher<{ crate::OFFSET }, { crate::PRIME }>;

/// FNV-0 variant hasher (deprecated in general use).
///
/// FNV-0 uses an initial state of `0` instead of a specific offset basis.
/// It is generally only used for historical purposes or specific hashing benchmarks.
pub type Fvn0Hasher = GenericFvn1Hasher<0, { crate::PRIME }>;

/// FNV-0 build hasher.
pub type Fvn0BuildHasher = GenericFvn1BuildHasher<0, { crate::PRIME }>;
