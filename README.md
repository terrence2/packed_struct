[![CI](https://codeberg.org/terrence/packed_struct/badges/workflows/qa.yaml/badge.svg)](https://codeberg.org/terrence/packed_struct/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://codeberg.org/terrence/packed_struct#LICENSE)
[![Crates.io](https://img.shields.io/crates/v/nitrogen_packed_struct.svg)](https://crates.io/crates/nitrogen_packed_struct)
[![Downloads](https://img.shields.io/crates/d/nitrogen_packed_struct.svg)](https://crates.io/crates/nitrogen_packed_struct)
[![Docs](https://docs.rs/nitrogen_packed_struct/badge.svg)](https://docs.rs/packed_struct/latest/nitrogen_packed_struct/)

# Packed Struct
---
A helper proc macro to make `#[repr(C, packed)]` ergonomic.

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
of a packed struct. That's a ton of boilerplate though; hence this crate.

## Example

```rust
fn test_packed_struct() {
    #[packed_struct]
    #[derive(Copy, Clone)]
    struct TestStructDerive {
        a: u8,
        b: u32,
        c: u16,
    }

    // Direct initialization
    let ts = TestStructDerive {
        a: 42,
        b: 1,
        c: 256,
    };

    // Expected size and alignment
    assert_eq!(size_of::<TestStructDerive>(), 7);
    assert_eq!(align_of::<TestStructDerive>(), 1);

    // Access members without warning.
    assert_eq!(ts.a(), 42u8);
    assert_eq!(ts.b(), 1u32);
    assert_eq!(ts.c(), 256u16);

    // Overlay items, slices, and prefix on a bytes buffer.
    let buf: &[u8] = &[42, 1, 0, 0, 0, 0, 1, 42, 2, 0, 0, 0, 0, 2];
    let ts1 = TestStructDerive::overlay(buf)?;
    let ts2 = TestStructDerive::overlay_prefix(buf)?;
    let ts3 = TestStructDerive::overlay_slice(buf5)?;
    assert_eq!(ts2.a(), 42u8);
    assert_eq!(ts2.b(), 1u32);
    assert_eq!(ts2.c(), 256u16);
    assert_eq!(ts3[1].a(), 42u8);
    assert_eq!(ts3[1].b(), 2u32);
    assert_eq!(ts3[1].c(), 512u16);
}
```

## Changes from 0.0.1 -> 0.2.0
* Relicensed as MIT
* Moved from a normal macro to a proc macro
* No longer need to add a dummy name to each field
* Now implements zerocopy and adds overlay methods
