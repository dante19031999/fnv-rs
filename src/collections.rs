#![cfg(feature = "std")]
//! 📦 Standard Collection Aliases
//!
//! This module provides type aliases for [`HashMap`] and [`HashSet`] using various FNV hashing variants.
//!
//! **Note: All types in this module are re-exported at the crate root.** You can use them
//! via `fnv64_rs::*` instead of `fnv64_rs::collection::*`.

use crate::{Fvn0BuildHasher, Fvn1BuildHasher, Fvn1aBuildHasher, FvnBuildHasher};
use std::collections::{HashMap, HashSet};

#[cfg(feature = "nightly")]
use std::alloc::{Allocator, Global};

// --- HashMap Aliases ---

#[cfg(not(feature = "nightly"))]
/// A [`HashMap`] using the **FNV-0** algorithm.
pub type Fvn0HashMap<K, V> = HashMap<K, V, Fvn0BuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashMap`] using the **FNV-0** algorithm with support for custom allocators.
pub type Fvn0HashMap<K, V, A = Global> = HashMap<K, V, Fvn0BuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashMap`] using the **FNV-1** algorithm.
pub type Fvn1HashMap<K, V> = HashMap<K, V, Fvn1BuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashMap`] using the **FNV-1** algorithm with support for custom allocators.
pub type Fvn1HashMap<K, V, A = Global> = HashMap<K, V, Fvn1BuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashMap`] using the **FNV-1a** algorithm.
pub type Fvn1aHashMap<K, V> = HashMap<K, V, Fvn1aBuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashMap`] using the **FNV-1a** algorithm with support for custom allocators.
pub type Fvn1aHashMap<K, V, A = Global> = HashMap<K, V, Fvn1aBuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashMap`] using the **default** FNV implementation (aliases **FNV-1a**).
pub type FvnHashMap<K, V> = HashMap<K, V, FvnBuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashMap`] using the **default** FNV implementation with support for custom allocators.
pub type FvnHashMap<K, V, A = Global> = HashMap<K, V, FvnBuildHasher, A>;

// --- HashSet Aliases ---

#[cfg(not(feature = "nightly"))]
/// A [`HashSet`] using the **FNV-0** algorithm.
pub type Fvn0HashSet<V> = HashSet<V, Fvn0BuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashSet`] using the **FNV-0** algorithm with support for custom allocators.
pub type Fvn0HashSet<V, A = Global> = HashSet<V, Fvn0BuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashSet`] using the **FNV-1** algorithm.
pub type Fvn1HashSet<V> = HashSet<V, Fvn1BuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashSet`] using the **FNV-1** algorithm with support for custom allocators.
pub type Fvn1HashSet<V, A = Global> = HashSet<V, Fvn1BuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashSet`] using the **FNV-1a** algorithm.
pub type Fvn1aHashSet<V> = HashSet<V, Fvn1aBuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashSet`] using the **FNV-1a** algorithm with support for custom allocators.
pub type Fvn1aHashSet<V, A = Global> = HashSet<V, Fvn1aBuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashSet`] using the **default** FNV implementation (aliases **FNV-1a**).
pub type FvnHashSet<V> = HashSet<V, FvnBuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashSet`] using the **default** FNV implementation with support for custom allocators.
pub type FvnHashSet<V, A = Global> = HashSet<V, FvnBuildHasher, A>;
