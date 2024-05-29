use byteorder::ByteOrder;
use core::{marker::PhantomData, mem::size_of};

use crate::XRefFinder;

/// Verifies that an absolute offset interpretation of `value` would lead the processor to `target`
pub const fn is_absolute_match(value: usize, target: usize) -> bool {
    // Kinda redundant, but kept for completeness
    value == target
}

#[cfg(target_pointer_width = "64")]
pub(crate) fn does_match_absolute<Endian: ByteOrder>(bytes: &[u8], offset: usize, target: usize) -> bool {
    let value = Endian::read_u64(&bytes[offset..offset + size_of::<u64>()]);
    is_absolute_match(value as usize, target)
}

#[cfg(target_pointer_width = "32")]
pub(crate) fn does_match_absolute<Endian: ByteOrder>(bytes: &[u8], offset: usize, target: usize) -> bool {
    let value = Endian::read_u32(&bytes[offset..offset + size_of::<u32>()]);
    is_absolute_match(value as usize, target)
}

pub struct AbsoluteFinder<Endian: ByteOrder> {
    target: usize,
    endian: PhantomData<Endian>,
}

impl<Endian: ByteOrder> AbsoluteFinder<Endian> {
    /// Creates a new AbsoluteFinder, that can then find absolute cross references
    ///
    /// Arguments:
    ///
    /// * `target`: The address, which the reference should point to
    pub fn new(target: usize) -> Self {
        Self {
            target,
            endian: PhantomData,
        }
    }
}

impl<Endian: ByteOrder> XRefFinder for AbsoluteFinder<Endian> {
    fn does_match(&self, bytes: &[u8], offset: usize) -> bool {
        if bytes.len() - offset < size_of::<usize>() {
            return false;
        }
        does_match_absolute::<Endian>(bytes, offset, self.target)
    }
}
