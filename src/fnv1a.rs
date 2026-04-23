//! 🦀 FNV-1a Hashing Implementation
//!
//! This module provides the **FNV-1a** variant of the Fowler–Noll–Vo hash function.
//!
//! **Note: All types in this module are re-exported at the crate root.** You can use them
//! via `fnv64_rs::*` instead of `fnv64_rs::fnv1a::*`.
//!
//! ### 💡 Algorithm Logic
//! FNV-1a is the most widely recommended variant of FNV. It differs from FNV-1 by using an
//! **XOR-then-Multiply** sequence, which significantly improves the "avalanche characteristics"
//! (how much the hash changes when a single bit of input changes).
//!
//! 1. Start with the [`OFFSET_BASIS`][`crate::OFFSET`].
//! 2. For each byte:
//!    * XOR the current hash with the byte.
//!    * Multiply the result by the [`FNV_PRIME`][`crate::PRIME`].
//!
//! ### 🛠️ Usage
//! This module provides the [`Fnv1aHasher`] and the [`Fnv1aBuildHasher`].
//! FNV-1a is the default choice for [`FnvHashMap`](crate::FnvHashMap).
//!
//! ```rust
//! use core::hash::Hasher;
//! use fnv64_rs::Fnv1aHasher;
//!
//! let mut hasher = Fnv1aHasher::default();
//! hasher.write(b"Rust");
//! let result = hasher.finish();
//! ```
use core::hash::{BuildHasher, Hasher};

/// A generic implementation of the FNV-1a (Fowler–Noll–Vo) hashing algorithm.
///
/// FNV-1a is a variation of the FNV-1 algorithm that changes the order of the
/// XOR and multiplication steps. This usually results in better "avalanche"
/// characteristics for short strings.
pub struct GenericFnv1aHasher<const OFFSET: u64, const PRIME: u64> {
    hash: u64,
}

impl<const OFFSET: u64, const PRIME: u64> Hasher for GenericFnv1aHasher<OFFSET, PRIME> {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    /// Performs the FNV-1a hash transformation: `hash = (hash ^ byte) * PRIME`.
    ///
    /// This implementation uses wrapping multiplication to ensure consistent
    /// behavior across debug and release profiles, as the algorithm relies
    /// on 64-bit integer overflow.
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            // FNV-1a: XOR then Multiply
            self.hash ^= *byte as u64;
            self.hash = self.hash.wrapping_mul(PRIME);
        }
    }
}

impl<const OFFSET: u64, const PRIME: u64> Default for GenericFnv1aHasher<OFFSET, PRIME> {
    /// Initializes the hasher with the provided `OFFSET` basis.
    fn default() -> Self {
        Self { hash: OFFSET }
    }
}

/// A builder for creating [`GenericFnv1aHasher`] instances with specific parameters.
///
/// This is useful for integrating with collections like `HashMap` or `HashSet`
/// that require a `BuildHasher` to initialize their internal hashing state.
pub struct GenericFnv1aBuildHasher<const OFFSET: u64, const PRIME: u64> {}

impl<const OFFSET: u64, const PRIME: u64> BuildHasher for GenericFnv1aBuildHasher<OFFSET, PRIME> {
    type Hasher = GenericFnv1aHasher<OFFSET, PRIME>;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        GenericFnv1aHasher::default()
    }
}

impl<const OFFSET: u64, const PRIME: u64> Default for GenericFnv1aBuildHasher<OFFSET, PRIME> {
    fn default() -> Self {
        Self {}
    }
}

/// Standard 64-bit FNV-1a hasher using the official FNV offset basis and prime.
///
/// This is the most common 64-bit FNV variant used in modern applications.
pub type Fnv1aHasher = GenericFnv1aHasher<{ crate::OFFSET }, { crate::PRIME }>;

/// Standard 64-bit FNV-1a build hasher.
///
/// Use this to initialize a `HashMap` with FNV-1a:
/// `let mut map: HashMap<K, V, Fnv1aBuildHasher> = HashMap::default();`
pub type Fnv1aBuildHasher = GenericFnv1aBuildHasher<{ crate::OFFSET }, { crate::PRIME }>;
