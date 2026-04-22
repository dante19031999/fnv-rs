use core::hash::{BuildHasher, Hasher};

/// A generic implementation of the FNV-1a (Fowler–Noll–Vo) hashing algorithm.
///
/// FNV-1a is a variation of the FNV-1 algorithm that changes the order of the
/// XOR and multiplication steps. This usually results in better "avalanche"
/// characteristics for short strings.
pub struct GenericFvn1aHasher<const OFFSET: u64, const PRIME: u64> {
    hash: u64,
}

impl<const OFFSET: u64, const PRIME: u64> Hasher for GenericFvn1aHasher<OFFSET, PRIME> {
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

impl<const OFFSET: u64, const PRIME: u64> Default for GenericFvn1aHasher<OFFSET, PRIME> {
    /// Initializes the hasher with the provided `OFFSET` basis.
    fn default() -> Self {
        Self { hash: OFFSET }
    }
}

/// A builder for creating [`GenericFvn1aHasher`] instances with specific parameters.
///
/// This is useful for integrating with collections like `HashMap` or `HashSet`
/// that require a `BuildHasher` to initialize their internal hashing state.
pub struct GenericFvn1aBuildHasher<const OFFSET: u64, const PRIME: u64> {}

impl<const OFFSET: u64, const PRIME: u64> BuildHasher for GenericFvn1aBuildHasher<OFFSET, PRIME> {
    type Hasher = GenericFvn1aHasher<OFFSET, PRIME>;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        GenericFvn1aHasher::default()
    }
}

impl<const OFFSET: u64, const PRIME: u64> Default for GenericFvn1aBuildHasher<OFFSET, PRIME> {
    fn default() -> Self {
        Self {}
    }
}

/// Standard 64-bit FNV-1a hasher using the official FNV offset basis and prime.
///
/// This is the most common 64-bit FNV variant used in modern applications.
pub type Fvn1aHasher = GenericFvn1aHasher<{ crate::OFFSET }, { crate::PRIME }>;

/// Standard 64-bit FNV-1a build hasher.
///
/// Use this to initialize a `HashMap` with FNV-1a:
/// `let mut map: HashMap<K, V, Fvn1aBuildHasher> = HashMap::default();`
pub type Fvn1aBuildHasher = GenericFvn1aBuildHasher<{ crate::OFFSET }, { crate::PRIME }>;
