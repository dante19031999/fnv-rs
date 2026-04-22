//! [![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/dante19031999/fnv-rs/tree/main?tab=Apache-2.0-1-ov-file#readme)
//! [![No Std](https://img.shields.io/badge/no__std-compatible-success.svg)](#)
//! [![Rust Version](https://img.shields.io/badge/rustc-1.56+-lightgray.svg?logo=rust)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)
//! [![Cargo Version](https://img.shields.io/crates/v/fnv64-rs.svg)](https://crates.io/crates/fnv64-rs)
//! [![Documentation](https://docs.rs/fnv64-rs/badge.svg)](https://docs.rs/fnv64-rs)
//!
//! # FNV Hashing
//!
//! An implementation of the [Fowler–Noll–Vo (FNV)](https://en.wikipedia.org/wiki/Fowler–Noll–Vo_hash_function)
//! non-cryptographic hash function.
//!
//! This crate provides the 64-bit versions of FNV-0, FNV-1, and FNV-1a.
//! FNV hashes are designed to be fast while maintaining a low collision rate,
//! making them ideal for use in lookup tables and hash maps.
//!
//! # 🦀 FVN64-RS
//! 
//! A lightweight, zero-dependency, `no_std` implementation of the 64-bit **Fowler–Noll–Vo (FNV)** non-cryptographic hash
//! function in Rust.
//! 
//! This crate provides high-performance implementations of the **FNV-0**, **FNV-1**, and **FNV-1a** algorithms using const
//! generics to ensure zero-cost abstractions.
//! 
//! ## 💡 Why FNV?
//! 
//! FNV is a non-cryptographic hash function created by Glenn Fowler, Landon Curt Noll, and Kiem-Phong Vo. It is designed to
//! be:
//! 
//! 1. Extremely fast to compute.
//! 2. Easy to implement (only multiplication and XOR).
//! 3. Low collision rate for common data types.
//! 
//! It is particularly effective for small data sets like property names in compilers, object keys in scripts, or small
//! strings.
//! 
//! ## 🚀 Features
//! 
//! - **Zero Dependencies**: Pure Rust implementation with no external requirements.
//! - **`no_std` Support**: Ideal for embedded systems and low-level kernel development.
//! - **Const Generics**: Uses modern Rust features to provide a generic interface for FNV variants.
//! - **Standard Library Integration**: Fully implements `core::hash::Hasher` and `core::hash::BuildHasher`.
//! 
//! ## 📦 Usage
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! fnv64-rs = "0.1.0"
//! ```
//! 
//! ## 🛠️ Using with HashMap
//! 
//! The FNV algorithm is frequently used in hash maps because of its speed and low collision rate for short keys (like
//! strings or integers).
//! 
//! ```rust
//! use std::collections::HashMap;
//! use fnv64_rs::FvnBuildHasher;
//! 
//! // Initialize a HashMap using the FNV-1a BuildHasher
//! let mut map: HashMap<String, i32, FvnBuildHasher> = HashMap::default();
//! 
//! map.insert("apple".to_string(), 1);
//! map.insert("orange".to_string(), 2);
//!
//! assert_eq!(map.get("apple"), Some(&1));
//! assert_eq!(map.get("orange"), Some(&2));
//! assert_eq!(map.get("pineapple"), None);
//! ```
//! 
//! ## 🛠️ Using with HashSet
//! 
//! The FNV algorithm is frequently used in hash sets because of its speed and low collision rate for short keys (like
//! strings or integers).
//! 
//! ```rust
//! use std::collections::HashSet;
//! use fnv64_rs::FvnBuildHasher;
//! 
//! // Initialize a HashMap using the FNV-1a BuildHasher
//! let mut set: HashSet<String, FvnBuildHasher> = HashSet::default();
//! 
//! set.insert("apple".to_string());
//! set.insert("orange".to_string());
//!
//! assert!(set.contains("apple"));
//! assert!(set.contains("orange"));
//! assert!(!set.contains("pineapple"));
//! ```
//! 
//! ## 🛠️ Direct Hashing
//! 
//! You can use the hasher directly to compute values for specific data:
//! 
//! ```rust
//! use core::hash::Hasher;
//! use fnv64_rs::Fvn1aHasher;
//! 
//! let mut hasher = Fvn1aHasher::default();
//! hasher.write(b"hello world");
//! let result = hasher.finish();
//! 
//! println!("Hash: {:x}", result);
//! ```
//! 
//! ## 📊 Algorithm Variants
//! 
//! | Variant     | Implementation          | Logic                   | Characteristics                                    |
//! |:------------|:------------------------|:------------------------|:---------------------------------------------------|
//! | **FNV-1a**  | `fvn64_rs::Fvn1aHasher` | `(hash ^ byte) * PRIME` | **Recommended.** Best avalanche characteristics.   |
//! | **FNV-1**   | `fvn64_rs::Fvn1Hasher`  | `(hash * PRIME) ^ byte` | The original FNV-1 specification.                  |
//! | **FNV-0**   | `fvn64_rs::Fvn0Hasher`  | `hash = 0; ...`         | Deprecated. Included for historical purposes.      |
//! | **DEFAULT** | `fvn64_rs::FvnHasher`   | `(hash ^ byte) * PRIME` | **Default** implementation. Implements **FNV-1a**. |
//! 
//! ## ⚖️ License
//! 
//! This project is licensed under
//! the [Apache-2.0 License](https://github.com/dante19031999/fnv-rs/tree/main?tab=Apache-2.0-1-ov-file#readme).

#![no_std]

mod constants;
mod fvn;
mod fvn1a;

pub use constants::*;
pub use fvn::*;
pub use fvn1a::*;

///! An alias for the default 64-bit FNV hasher.
///
///! This currently points to [`Fvn1aHasher`], which is the recommended
///! variant for most applications due to its superior bit-dispersion properties.
pub use fvn1a::Fvn1aHasher as FvnHasher;

///! An alias for the default 64-bit FNV build hasher.
///
///! This provides a convenient way to use FNV with standard library collections:
///
///! ```rust
///! use std::collections::HashMap;
///! use fnv64_rs::FvnBuildHasher;
///
///! let mut map: HashMap<String, i32, FvnBuildHasher> = HashMap::default();
///! map.insert("apple".to_string(), 1);
///! ```
pub use fvn1a::Fvn1aBuildHasher as FvnBuildHasher;
