/// The FNV offset basis for 64-bit FNV hashing.
///
/// This is the initial value used to start the hashing process. It ensures
/// that the hash state begins with a non-zero value to improve bit distribution.
pub const OFFSET: u64 = 0xcbf29ce484222325;

/// The FNV prime value for 64-bit FNV hashing.
///
/// This prime is used in the multiplication step of the hash function:
/// `hash *= PRIME`. It was chosen to provide high dispersion
/// and minimize collisions.
pub const PRIME: u64 = 0x00000100000001b3;
