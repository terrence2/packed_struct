# Packed Struct
**A macro to make it easier to use `#[repr(packed)]` structs in Rust.**

## Motivation
Many network and file serialization formats specify a strict layout in
memory. With such formats, it is it is frequently possible (not to mention
faster and easier) to just cast a raw pointer to the data into a structure
with the known shape and access through a pointer of the right type.

By default, Rust does not make promises about struct layout (for very good
reasons), but when needed for the above purpose, you can force Rust to give
you an exact layout by tagging it with `#[repr(packed)]`. There is a rather
bad downside however: when Rust cannot control the layout, it cannot make
guarantees about the alignment of data within the struct. While x86(_64) and
ARM are nice about letting you access memory at arbitrary alignments, most
processors are not so nice and will just hard-interrupt if you try.

Thus, Rust cannot promise that types stored in such a struct will behave the
same as the same type stored outside of such a struct on all architectures.
To ensure that appropriate masking and shifting always happens, Rust enforces
rules around access to members of packed structs that disable most any access
other than copies out of that memory.

This is easy to work around: we just need to add an accessor for all members
of a packed struct. That's a ton of boilerplate though; hence this macro.

## Example

```rust
// Each field in the struct contains three required fields: the actual member
// name, the accessor name, and the member type. If the accessor should return a
// different type, you can express the type as `physical_type as return_type`.
packed_struct!(COFFHeader {
    _0 => machine: u16,
    _1 => number_of_sections: u16,
    _2 => time_date_stamp: u32,
    _3 => pointer_to_symbol_table: u32,
    _4 => number_of_symbols: u32,
    _5 => size_of_optional_header: u16 as usize,
    _6 => characteristics: u16
});

extern crate failure;
use failure::Error;

fn optional_header_size(data: &[u8]) -> Result<usize, Error> {
    let coff = COFFHeader::overlay(data)?;
    return Ok(coff.size_of_optional_header());
}
```
