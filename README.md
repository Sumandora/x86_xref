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
find_xref_abs::<NativeEndian>(&bytes, 0xDEADBEEF); // find the next absolute xref
find_xref_rel::<NativeEndian>(&bytes, bytes.as_ptr() as usize, 4, 0xDEADBEEF); // find the next relative xref
find_xref::<NativeEndian>(&bytes, bytes.as_ptr() as usize, 4, 0xDEADBEEF); // find the next xref
```
