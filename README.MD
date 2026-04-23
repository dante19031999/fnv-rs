[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/dante19031999/fnv-rs/tree/main?tab=Apache-2.0-1-ov-file#readme)
[![No Std](https://img.shields.io/badge/no__std-compatible-success.svg)](#)
[![Rust Version](https://img.shields.io/badge/rustc-1.56+-lightgray.svg?logo=rust)](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html)
[![Cargo Version](https://img.shields.io/crates/v/fnv64-rs.svg)](https://crates.io/crates/fnv64-rs)
[![Documentation](https://docs.rs/fnv64-rs/badge.svg)](https://docs.rs/fnv64-rs)

# 🦀 FNV64-RS

A lightweight, zero-dependency, `no_std` implementation of the 64-bit **Fowler–Noll–Vo (FNV)** non-cryptographic hash
function in Rust.

This crate provides high-performance implementations of the **FNV-0**, **FNV-1**, and **FNV-1a** algorithms using const
generics to ensure zero-cost abstractions.

## 💡 Why FNV?

FNV is a non-cryptographic hash function created by Glenn Fowler, Landon Curt Noll, and Kiem-Phong Vo. It is designed to
be:

1. Extremely fast to compute.
2. Easy to implement (only multiplication and XOR).
3. Low collision rate for common data types.

It is particularly effective for small data sets like property names in compilers, object keys in scripts, or small
strings.

## 🚀 Features

- **Zero Dependencies**: Pure Rust implementation with no external requirements.
- **`no_std` Support**: Ideal for embedded systems and low-level kernel development.
- **Const Generics**: Uses modern Rust features to provide a generic interface for FNV variants.
- **Standard Library Integration**: Fully implements `core::hash::Hasher` and `core::hash::BuildHasher`.

## 📦 Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
fnv64-rs = "0.1.0"
```

## 🛠️ Using with HashMap

The FNV algorithm is frequently used in hash maps because of its speed and low collision rate for short keys (like
strings or integers).

```rust
use std::collections::HashMap;
use fnv64_rs::FnvBuildHasher;

// Initialize a HashMap using the FNV-1a BuildHasher
let mut map: HashMap<String, i32, FnvBuildHasher> = HashMap::default();

map.insert("apple".to_string(), 1);
map.insert("orange".to_string(), 2);

assert_eq!(map.get("apple"), Some(&1));
assert_eq!(map.get("orange"), Some(&2));
assert_eq!(map.get("pineapple"), None);

```

## 🛠️ Using with HashSet

The FNV algorithm is frequently used in hash sets because of its speed and low collision rate for short keys (like
strings or integers).

```rust
use std::collections::HashMap;
use fnv64_rs::FnvBuildHasher;

// Initialize a HashSet using the FNV-1a BuildHasher
let mut set: HashSet<String, FnvBuildHasher> = HashSet::default();

set.insert("apple".to_string());
set.insert("orange".to_string());

assert!(set.contains("apple"));
assert!(set.contains("orange"));
assert!(!set.contains("pineapple"));

```

## 🛠️ Direct Hashing

You can use the hasher directly to compute values for specific data:

```rust
use core::hash::Hasher;
use fnv64_rs::Fnv1aHasher;

let mut hasher = Fnv1aHasher::default();
hasher.write(b"hello world");
let result = hasher.finish();

println!("Hash: {:x}", result);
```

## 📦 Collections (Standard Library)

[![Feature: std](https://img.shields.io/badge/feature-std-yellow.svg)](https://doc.rust-lang.org/std/)

When the std feature is enabled, fnv64-rs provides convenient type aliases for standard library collections. These are
pre-configured with FNV hashers and re-exported at the crate root for a seamless developer experience.

### Available Aliases

| Type                 | Hash Algorithm       | Collection |
|:---------------------|:---------------------|:-----------|
| `FnvHashMap<K, V>`   | **FNV-1a** (Default) | `HashMap`  |
| `FnvHashSet<V>`      | **FNV-1a** (Default) | `HashSet`  |
| `Fnv1aHashMap<K, V>` | **FNV-1a**           | `HashMap`  |
| `Fnv1aHashSet<V>`    | **FNV-1a**           | `HashSet`  |
| `Fnv1HashMap<K, V>`  | **FNV-1**            | `HashMap`  |
| `Fnv1HashSet<V>`     | **FNV-1**            | `HashSet`  |
| `Fnv0HashMap<K, V>`  | **FNV-0**            | `HashMap`  |
| `Fnv0HashSet<V>`     | **FNV-0**            | `HashSet`  |

> [!TIP]
> On **Nightly Rust**, these aliases also support the **Allocator API**, allowing you to use FNV collections with custom
> memory allocators. Use `--features nightly` to enable this.

### 🛠️ Example: Drop-in Replacement

You can use these aliases directly without needing to manually import or configure a BuildHasher:

```rust
use fnv64_rs::{FnvHashMap, FnvHashSet};

// No complex generic boilerplate needed!
let mut map = FnvHashMap::default();
map.insert("id_01", 100);

let mut set = FnvHashSet::default();
set.insert("admin");
```

## 📊 Algorithm Variants

| Variant     | Implementation          | Logic                   | Characteristics                                    |
|:------------|:------------------------|:------------------------|:---------------------------------------------------|
| **FNV-1a**  | `fnv64_rs::Fnv1aHasher` | `(hash ^ byte) * PRIME` | **Recommended.** Best avalanche characteristics.   |
| **FNV-1**   | `fnv64_rs::Fnv1Hasher`  | `(hash * PRIME) ^ byte` | The original FNV-1 specification.                  |
| **FNV-0**   | `fnv64_rs::Fnv0Hasher`  | `hash = 0; ...`         | Deprecated. Included for historical purposes.      |
| **DEFAULT** | `fnv64_rs::FnvHasher`   | `(hash ^ byte) * PRIME` | **Default** implementation. Implements **FNV-1a**. |

## ⚖️ License

This project is licensed under
the [Apache-2.0 License](https://github.com/dante19031999/fnv-rs/tree/main?tab=Apache-2.0-1-ov-file#readme).



