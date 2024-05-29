use core::{marker::PhantomData, mem::size_of};

use byteorder::ByteOrder;

use crate::{absolute_finder::does_match_absolute, XRefFinder};

pub struct RelativeAndAbsoluteFinder<Endian: ByteOrder> {
    base_address: usize,
    instruction_length: usize,
    target: usize,
    endian: PhantomData<Endian>,
}

impl<Endian: ByteOrder> RelativeAndAbsoluteFinder<Endian> {
    /// Creates a new RelativeAndAbsoluteFinder, that can then find relative and absolute cross references
    ///
    /// For arguments refer to `RelativeFinder` and `AbsoluteFinder`
    pub fn new(base_address: usize, instruction_length: usize, target: usize) -> Self {
        Self {
            base_address,
            instruction_length,
            target,
            endian: PhantomData,
        }
    }
}

impl<Endian: ByteOrder> XRefFinder for RelativeAndAbsoluteFinder<Endian> {
    fn does_match(&self, bytes: &[u8], offset: usize) -> bool {
        use crate::relative_finder::does_match_relative;

        if bytes.len() - offset >= size_of::<i32>()
            && does_match_relative::<Endian>(
                bytes,
                offset,
                self.base_address,
                self.instruction_length,
                self.target,
            )
        {
            return true;
        }

        if bytes.len() - offset >= size_of::<usize>()
            && does_match_absolute::<Endian>(bytes, offset, self.target)
        {
            return true;
        }

        false
    }
}
