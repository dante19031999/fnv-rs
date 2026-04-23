#![cfg(feature = "std")]
//! 📦 Standard Collection Aliases
//!
//! This module provides type aliases for [`HashMap`] and [`HashSet`] using various FNV hashing variants.
//!
//! **Note: All types in this module are re-exported at the crate root.** You can use them
//! via `fnv64_rs::*` instead of `fnv64_rs::collection::*`.

use crate::{Fnv0BuildHasher, Fnv1BuildHasher, Fnv1aBuildHasher, FnvBuildHasher};
use std::collections::{HashMap, HashSet};

#[cfg(feature = "nightly")]
use std::alloc::{Allocator, Global};

// --- HashMap Aliases ---

#[cfg(not(feature = "nightly"))]
/// A [`HashMap`] using the **FNV-0** algorithm.
pub type Fnv0HashMap<K, V> = HashMap<K, V, Fnv0BuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashMap`] using the **FNV-0** algorithm with support for custom allocators.
pub type Fnv0HashMap<K, V, A = Global> = HashMap<K, V, Fnv0BuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashMap`] using the **FNV-1** algorithm.
pub type Fnv1HashMap<K, V> = HashMap<K, V, Fnv1BuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashMap`] using the **FNV-1** algorithm with support for custom allocators.
pub type Fnv1HashMap<K, V, A = Global> = HashMap<K, V, Fnv1BuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashMap`] using the **FNV-1a** algorithm.
pub type Fnv1aHashMap<K, V> = HashMap<K, V, Fnv1aBuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashMap`] using the **FNV-1a** algorithm with support for custom allocators.
pub type Fnv1aHashMap<K, V, A = Global> = HashMap<K, V, Fnv1aBuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashMap`] using the **default** FNV implementation (aliases **FNV-1a**).
pub type FnvHashMap<K, V> = HashMap<K, V, FnvBuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashMap`] using the **default** FNV implementation with support for custom allocators.
pub type FnvHashMap<K, V, A = Global> = HashMap<K, V, FnvBuildHasher, A>;

// --- HashSet Aliases ---

#[cfg(not(feature = "nightly"))]
/// A [`HashSet`] using the **FNV-0** algorithm.
pub type Fnv0HashSet<V> = HashSet<V, Fnv0BuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashSet`] using the **FNV-0** algorithm with support for custom allocators.
pub type Fnv0HashSet<V, A = Global> = HashSet<V, Fnv0BuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashSet`] using the **FNV-1** algorithm.
pub type Fnv1HashSet<V> = HashSet<V, Fnv1BuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashSet`] using the **FNV-1** algorithm with support for custom allocators.
pub type Fnv1HashSet<V, A = Global> = HashSet<V, Fnv1BuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashSet`] using the **FNV-1a** algorithm.
pub type Fnv1aHashSet<V> = HashSet<V, Fnv1aBuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashSet`] using the **FNV-1a** algorithm with support for custom allocators.
pub type Fnv1aHashSet<V, A = Global> = HashSet<V, Fnv1aBuildHasher, A>;

#[cfg(not(feature = "nightly"))]
/// A [`HashSet`] using the **default** FNV implementation (aliases **FNV-1a**).
pub type FnvHashSet<V> = HashSet<V, FnvBuildHasher>;
#[cfg(feature = "nightly")]
/// A [`HashSet`] using the **default** FNV implementation with support for custom allocators.
pub type FnvHashSet<V, A = Global> = HashSet<V, FnvBuildHasher, A>;
