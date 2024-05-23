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
//! find_xref_abs::<NativeEndian>(&bytes, 0xDEADBEEF); // find the next absolute xref
//! find_xref_rel::<NativeEndian>(&bytes, bytes.as_ptr() as usize, 4, 0xDEADBEEF); // find the next relative xref
//! find_xref::<NativeEndian>(&bytes, bytes.as_ptr() as usize, 4, 0xDEADBEEF); // find the next xref
//! ```
//! 

#![no_std]

use byteorder::ByteOrder;
use core::mem::size_of;

/// Verifies that an relative offset interpretation of `base_address`, `instruction_length` and `offset` would lead the processor to `target`
pub fn is_relative_match(
    address: usize,
    instruction_length: usize,
    offset: isize,
    target: usize,
) -> bool {
    let mut address = address + instruction_length;
    if offset.is_negative() {
        address -= offset.abs() as usize;
    } else {
        address += offset.abs() as usize;
    }
    return address == target;
}

/// Verifies that an absolute offset interpretation of `value` would lead the processor to `target`
pub fn is_absolute_match(value: usize, target: usize) -> bool {
    // Kinda redundant, but kept for completeness
    return value == target;
}

#[cfg(target_pointer_width = "64")]
fn does_match_relative<Endian: ByteOrder>(
    bytes: &[u8],
    offset: usize,
    base_address: usize,
    instruction_length: usize,
    target: usize,
) -> bool {
    let value = Endian::read_i32(&bytes[offset..offset + size_of::<i32>()]);
    return is_relative_match(
        base_address + offset,
        instruction_length,
        value as isize,
        target,
    );
}

#[cfg(target_pointer_width = "64")]
fn does_match_absolute<Endian: ByteOrder>(bytes: &[u8], offset: usize, target: usize) -> bool {
    let value = Endian::read_u64(&bytes[offset..offset + size_of::<u64>()]);
    return is_absolute_match(value as usize, target);
}

#[cfg(target_pointer_width = "32")]
fn does_match_absolute<Endian: ByteOrder>(bytes: &[u8], offset: usize, target: usize) -> bool {
    let value = Endian::read_u32(&bytes[offset..offset + size_of::<u32>()]);
    return is_absolute_match(value as usize, target);
}

/// Finds the first absolute reference in a `u8` slice
/// 
/// Arguments:
/// 
/// * `bytes`: Bytes to search through
/// * `target`: The address, which the reference should point to
pub fn find_xref_abs<Endian: ByteOrder>(bytes: &[u8], target: usize) -> Option<usize> {
    let len = bytes.len();
    if len < size_of::<usize>() {
        return None;
    }

    for i in 0..=len - size_of::<usize>() {
        if does_match_absolute::<Endian>(bytes, i, target) {
            return Some(i);
        }
    }

    return None;
}

/// Finds the first relative reference in a `u8` slice
/// 
/// Arguments:
/// 
/// * `bytes`: Bytes to search through
/// * `base_address`: Base address of relative references, this is useful when the memory you are scanning has been moved.
/// * `instruction_length`: The amount of bytes to skip from the relative offset.
///                         Most instructions, that use relative offsets end in the relative offset,
///                         so this is the size of the relative offset type (`i32`; `size_of::<i32>` = 4)
///                         If a instruction has the relative offset in the middle (e.g. cmp) then you need to set this to
///                         `size_of::<i32>` + how many bytes come after the relative offset
///                         Example: 48 83 3D [EF BE 00 00] 00    cmp $0x0, 0xBEEF(%rip) ; square brackets indicate relative offset
///                         here there is an additonal byte after the relative offset -> `instruction_length` = `size_of::<i32>` + 1 = 5
/// * `target`: The address, which the reference should point to
#[cfg(target_pointer_width = "64")]
pub fn find_xref_rel<Endian: ByteOrder>(
    bytes: &[u8],
    base_address: usize,
    instruction_length: usize,
    target: usize,
) -> Option<usize> {
    let len = bytes.len();
    if len < size_of::<i32>() {
        return None;
    }

    for i in 0..=len - size_of::<i32>() {
        if does_match_relative::<Endian>(bytes, i, base_address, instruction_length, target) {
            return Some(i);
        }
    }

    return None;
}

/// Finds the first reference in a `u8` slice
/// 
/// Arguments:
/// 
/// * `bytes`: Bytes to search through
/// * `base_address`: Base address of relative references, this is useful when the memory you are scanning has been moved.
/// * `instruction_length`: The amount of bytes to skip from the relative offset.
///                         Most instructions, that use relative offsets end in the relative offset,
///                         so this is the size of the relative offset type (`i32`; `size_of::<i32>` = 4)
///                         If a instruction has the relative offset in the middle (e.g. cmp) then you need to set this to
///                         `size_of::<i32>` + how many bytes come after the relative offset
///                         Example: 48 83 3D [EF BE 00 00] 00    cmp $0x0, 0xBEEF(%rip) ; square brackets indicate relative offset
///                         here there is an additonal byte after the relative offset -> `instruction_length` = `size_of::<i32>` + 1 = 5
/// * `target`: The address, which the reference should point to
#[cfg(target_pointer_width = "64")]
pub fn find_xref<Endian: ByteOrder>(
    bytes: &[u8],
    base_address: usize,
    instruction_length: usize,
    target: usize,
) -> Option<usize> {
    let len = bytes.len();

    for i in 0..=len {
        let remaining = len - i;

        if remaining >= size_of::<i32>()
            && does_match_relative::<Endian>(bytes, i, base_address, instruction_length, target)
        {
            return Some(i);
        }

        if remaining >= size_of::<u64>() && does_match_absolute::<Endian>(bytes, i, target) {
            return Some(i);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use byteorder::LittleEndian;

    use super::*;

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn check_find_xref() {
        let bytes = [0x00u8, 0x01, 0x23, 0x45, 0x67];

        assert_eq!(
            find_xref::<LittleEndian>(&bytes, 0, 4, 0x67452301 + 5),
            Some(1)
        );
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn check_find_xref_rel() {
        let bytes = [0x00u8, 0x01, 0x23, 0x45, 0x67];

        assert_eq!(
            find_xref_rel::<LittleEndian>(&bytes, 0, 4, 0x67452301 + 5),
            Some(1)
        );
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn check_find_xref_abs() {
        let bytes = [0x00u8, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];

        assert_eq!(
            find_xref_abs::<LittleEndian>(&bytes, 0xEFCDAB8967452301),
            Some(1)
        );
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn check_find_xref_abs_32_bit() {
        let bytes = [0x00u8, 0x01, 0x23, 0x45, 0x67];

        assert_eq!(find_xref_abs::<LittleEndian>(&bytes, 0x67452301), Some(1));
    }
}
