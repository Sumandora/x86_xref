use byteorder::ByteOrder;
use core::{marker::PhantomData, mem::size_of};

use crate::XRefFinder;

/// Verifies that an relative offset interpretation of `base_address`, `instruction_length` and `offset` would lead the processor to `target`
pub const fn is_relative_match(
    address: usize,
    instruction_length: usize,
    offset: isize,
    target: usize,
) -> bool {
    let mut address = address + instruction_length;
    if offset.is_negative() {
        address -= offset.unsigned_abs();
    } else {
        address += offset.unsigned_abs();
    }
    address == target
}

pub(crate) fn does_match_relative<Endian: ByteOrder>(
    bytes: &[u8],
    offset: usize,
    base_address: usize,
    instruction_length: usize,
    target: usize,
) -> bool {
    let value = Endian::read_i32(&bytes[offset..offset + size_of::<i32>()]);
    is_relative_match(
        base_address + offset,
        instruction_length,
        value as isize,
        target,
    )
}

pub struct RelativeFinder<Endian: ByteOrder> {
    base_address: usize,
    instruction_length: usize,
    target: usize,
    endian: PhantomData<Endian>,
}

impl<Endian: ByteOrder> RelativeFinder<Endian> {
    /// Creates a new RelativeFinder, that can then find relative cross references
    ///
    /// Arguments:
    ///
    /// * `base_address`: Base address of relative references, this is useful when the memory you are scanning has been moved.
    /// * `instruction_length`: The amount of bytes to skip from the relative offset.
    ///                         Most instructions, that use relative offsets, end in the relative offset,
    ///                         so this is the size of the relative offset type (`i32`; `size_of::<i32>` = 4)
    ///                         If a instruction has the relative offset in the middle (e.g. cmp) then you need to set this to
    ///                         `size_of::<i32>` + how many bytes come after the relative offset.
    ///                         Example: 48 83 3D [EF BE 00 00] 00    cmp $0x0, 0xBEEF(%rip) ; square brackets indicate relative offset
    ///                         here there is an additonal byte after the relative offset -> `instruction_length` = `size_of::<i32>` + 1 = 5
    /// * `target`: The address, which the reference should point to
    pub fn new(base_address: usize, instruction_length: usize, target: usize) -> Self {
        Self {
            base_address,
            instruction_length,
            target,
            endian: PhantomData,
        }
    }
}

impl<Endian: ByteOrder> XRefFinder for RelativeFinder<Endian> {
    fn does_match(&self, bytes: &[u8], offset: usize) -> bool {
        let i32_size = size_of::<i32>();
        if bytes.len() - offset < i32_size {
            return false;
        }
        does_match_relative::<Endian>(
            bytes,
            offset,
            self.base_address,
            self.instruction_length,
            self.target,
        )
    }
}
