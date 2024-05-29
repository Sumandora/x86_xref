//! # x86_ref
//!
//! x86_ref is a extremely minimal crate offering functions to find references in x86 binaries.
//!
//! ## Supported references
//!
//! - Absolute references on 32 and 64 bit
//! - Relative references on 64 bit (32 bit is omitted due to their rarity)
//!
//! ## Examples
//!
//! ```rust
//! use x86_xref::*;
//! use byteorder::NativeEndian;
//!
//! # let bytes = [0x00u8];
//! // let bytes: [u8];
//! AbsoluteFinder::<NativeEndian>::new(0xDEADBEEF).next(&bytes); // find the next absolute xref
//! RelativeFinder::<NativeEndian>::new(bytes.as_ptr() as usize, 4, 0xDEADBEEF).next(&bytes); // find the next relative xref
//! RelativeAndAbsoluteFinder::<NativeEndian>::new(bytes.as_ptr() as usize, 4, 0xDEADBEEF).next(&bytes); // find the next xref
//! ```
//!

#![cfg_attr(not(test), no_std)]

pub mod absolute_finder;
#[cfg(target_pointer_width = "64")]
pub mod relative_and_absolute_finder;
#[cfg(target_pointer_width = "64")]
pub mod relative_finder;

pub trait XRefFinder {
    /// Checks if the `offset` in `bytes` is a reference
    fn does_match(&self, bytes: &[u8], offset: usize) -> bool;

    /// Finds the next reference
    fn next(&self, bytes: &[u8]) -> Option<usize> {
        (0..=bytes.len()).find(|&i| self.does_match(bytes, i))
    }

    /// Finds the previous reference
    fn prev(&self, bytes: &[u8]) -> Option<usize> {
        (0..=bytes.len())
            .rev()
            .find(|&i| self.does_match(bytes, i))
            .map(|offset| bytes.len() - offset - 1)
    }

    // Finds all references in the `bytes` slice
    fn all(&self, bytes: &[u8]) -> impl Iterator<Item = usize> {
        (0..=bytes.len()).filter(|&i| self.does_match(bytes, i))
    }
}

pub use absolute_finder::AbsoluteFinder;
#[cfg(target_pointer_width = "64")]
pub use relative_and_absolute_finder::RelativeAndAbsoluteFinder;
#[cfg(target_pointer_width = "64")]
pub use relative_finder::RelativeFinder;

#[cfg(test)]
mod tests {
    use byteorder::LittleEndian;

    use super::*;

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn check_find_xref() {
        let bytes = [0x00u8, 0x01, 0x23, 0x45, 0x67];
        let searcher = RelativeAndAbsoluteFinder::<LittleEndian>::new(0, 4, 0x67452301 + 5);

        assert_eq!(searcher.next(&bytes), Some(1));
        assert_eq!(searcher.prev(&bytes), Some(3));
        assert_eq!(searcher.all(&bytes).collect::<Vec<_>>(), [1]);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn check_find_xref_rel() {
        let bytes = [0x00u8, 0x01, 0x23, 0x45, 0x67];
        let searcher = RelativeFinder::<LittleEndian>::new(0, 4, 0x67452301 + 5);

        assert_eq!(searcher.next(&bytes), Some(1));
        assert_eq!(searcher.prev(&bytes), Some(3));
        assert_eq!(searcher.all(&bytes).collect::<Vec<_>>(), [1]);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn check_find_xref_abs() {
        let bytes = [0x00u8, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        let searcher = AbsoluteFinder::<LittleEndian>::new(0xEFCDAB8967452301);

        assert_eq!(searcher.next(&bytes), Some(1));
        assert_eq!(searcher.prev(&bytes), Some(7));
        assert_eq!(searcher.all(&bytes).collect::<Vec<_>>(), [1]);
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn check_find_xref_abs_32_bit() {
        let bytes = [0x00u8, 0x01, 0x23, 0x45, 0x67];
        let searcher = AbsoluteFinder::<LittleEndian>::new(0x67452301);

        assert_eq!(searcher.next(&bytes), Some(1));
        assert_eq!(searcher.prev(&bytes), Some(3));
        assert_eq!(searcher.all(&bytes).collect::<Vec<_>>(), [1]);
    }
}
