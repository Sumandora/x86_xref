# x86_ref
x86_ref is a extremely minimal crate offering functions to find references in x86 binaries.

## Supported references

- Absolute references on 32 and 64 bit
- Relative references on 64 bit (32 bit is omitted due to their rarity)

## Examples

```rust
use x86_xref::*;
use byteorder::NativeEndian;

let bytes = ...;

AbsoluteFinder::<NativeEndian>::new(0xDEADBEEF).next(&bytes); // find the next absolute xref
RelativeFinder::<NativeEndian>::new(bytes.as_ptr() as usize, 4, 0xDEADBEEF).next(&bytes); // find the next relative xref
RelativeAndAbsoluteFinder::<NativeEndian>::new(bytes.as_ptr() as usize, 4, 0xDEADBEEF).next(&bytes); // find the next xref
```
