//! Zero-cost immutability wrapper for data leaving the storage boundary.
//!
//! `Frozen<T>` wraps a value and provides read-only access via `Deref`. It does
//! NOT implement `DerefMut`, so callers cannot mutate the inner value without
//! explicitly calling `.thaw()` to take ownership.
//!
//! This enforces the FP principle: **data is immutable by default at the storage
//! boundary; mutation requires explicit opt-in**.
//!
//! # Examples
//!
//! ```
//! use orqa_storage::Frozen;
//!
//! let projects: Frozen<Vec<String>> = Frozen::new(vec!["a".into(), "b".into()]);
//!
//! // Reading is free — Deref provides &Vec<String>
//! assert_eq!(projects.len(), 2);
//! assert_eq!(projects[0], "a");
//!
//! // Mutation requires explicit thaw
//! let mut owned = projects.thaw();
//! owned.push("c".into());
//! ```

use std::fmt;
use std::ops::Deref;

/// An immutable wrapper around a value of type `T`.
///
/// Provides `Deref<Target = T>` for read access but no `DerefMut`.
/// To mutate, call `.thaw()` which consumes the wrapper and returns
/// the owned inner value.
///
/// Implements `Clone`, `Debug`, `PartialEq`, `Eq`, `Hash`, `Serialize`
/// transparently — the wrapper is invisible to serialization and comparison.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Frozen<T>(T);

impl<T> Frozen<T> {
    /// Wrap a value, making it immutable at the type level.
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// Consume the wrapper and return the owned inner value.
    ///
    /// This is the only way to get a mutable `T` — an explicit opt-in
    /// that signals "I am intentionally breaking the immutability boundary".
    pub fn thaw(self) -> T {
        self.0
    }

    /// Get a shared reference to the inner value.
    pub fn inner(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Frozen<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: fmt::Debug> fmt::Debug for Frozen<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for Frozen<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: serde::Serialize> serde::Serialize for Frozen<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<T> From<T> for Frozen<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_provides_read_access() {
        let frozen = Frozen::new(vec![1, 2, 3]);
        assert_eq!(frozen.len(), 3);
        assert_eq!(frozen[0], 1);
    }

    #[test]
    fn thaw_returns_owned_value() {
        let frozen = Frozen::new(vec![1, 2, 3]);
        let mut owned = frozen.thaw();
        owned.push(4);
        assert_eq!(owned, vec![1, 2, 3, 4]);
    }

    #[test]
    fn clone_is_independent() {
        let a = Frozen::new(String::from("hello"));
        let b = a.clone();
        assert_eq!(*a, *b);
    }

    #[test]
    fn serialize_is_transparent() {
        let frozen = Frozen::new(42_i32);
        let json = serde_json::to_string(&frozen).expect("serialize");
        assert_eq!(json, "42");
    }
}
