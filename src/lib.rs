//! A small string type with fixed capacity stored on the stack
//!
//! # Examples
//!
//! ```
//! use fixstr::FixStr;
//!
//! // Create a FixStr with capacity of 16 octets
//! let tiny: FixStr<16> = FixStr::new("Hello").unwrap();
//! assert_eq!(tiny.as_str(), "Hello");
//! assert_eq!(tiny.capacity(), 16);
//!
//! // FixStr implements common traits
//! let tiny2: FixStr<16> = "World".try_into().unwrap();
//! let message: String = tiny2.into();
//! ```

/// A fixed-capacity string stored on the stack.
///
/// `FixStr<N>` stores up to N octets inline and guarantees valid UTF-8.
/// Useful for small strings where heap allocation is undesirable.
use std::fmt;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct FixStr<const N: usize> {
    inline: [u8; N],
    len: u8,
    _marker: PhantomData<[u8; N]>,
}

impl<const N: usize> FixStr<N> {
    /// Creates a new `FixStr` if the input fits within capacity.
    ///
    /// Returns `None` if the string is too long (> N octets) or exceeds `u8::MAX`.
    #[must_use]
    pub fn new(s: &str) -> Option<Self> {
        if s.len() > N || s.len() > u8::MAX as usize {
            return None;
        }

        // UTF-8 validation not needed here since &str is already guaranteed
        // to be valid UTF-8 by Rust's type system

        let mut buffer = [0u8; N];
        buffer[..s.len()].copy_from_slice(s.as_bytes());

        u8::try_from(s.len()).ok().map(|len| Self {
            inline: buffer,
            len,
            _marker: PhantomData,
        })
    }

    /// Creates a new `FixStr` without capacity checking.
    ///
    /// # Panics
    /// Panics if the string is too long for the fixed capacity.
    #[must_use]
    pub fn new_unchecked(s: &str) -> Self {
        Self::new(s)
            .unwrap_or_else(|| panic!("String '{s}' (len={}) exceeds capacity {N}", s.len()))
    }

    /// Returns a string slice containing the entire string.
    ///
    /// # Safety
    /// Safe because we only store valid UTF-8 strings.
    #[must_use]
    pub fn as_str(&self) -> &str {
        // SAFETY: We only store valid UTF-8 strings
        unsafe { std::str::from_utf8_unchecked(&self.inline[..self.len as usize]) }
    }

    /// Returns the length of the string in Unicode characters.
    ///
    /// This may be different from the octet length for non-ASCII strings.
    #[must_use]
    pub fn char_len(&self) -> usize {
        self.as_str().chars().count()
    }

    /// Returns the length of the string in octets.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns true if the string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the total capacity in octets.
    #[must_use]
    pub fn capacity(&self) -> usize {
        N
    }
}

impl<const N: usize> TryFrom<&str> for FixStr<N> {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::new(s).ok_or(format!(
            "String '{s}' (len={}) exceeds capacity {N}",
            s.len()
        ))
    }
}

impl<const N: usize> TryFrom<String> for FixStr<N> {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl<const N: usize> From<FixStr<N>> for String {
    fn from(s: FixStr<N>) -> Self {
        String::from(s.as_str())
    }
}

impl<const N: usize> AsRef<str> for FixStr<N> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> fmt::Display for FixStr<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
