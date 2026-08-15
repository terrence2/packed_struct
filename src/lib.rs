///
/// # Packed Struct
/// ---
/// A helper proc macro to make `#[repr(C, packed)]` ergonomic.
///
/// ## Motivation
/// Many network and file serialization formats specify a strict layout in
/// memory. With such formats, it is it is frequently possible (not to mention
/// faster and easier) to just cast a raw pointer to the data into a structure
/// with the known shape and access through a pointer of the right type.
///
/// By default, Rust does not make promises about struct layout (for very good
/// reasons), but when needed for the above purpose, you can force Rust to give
/// you an exact layout by tagging it with `#[repr(packed)]`. There is a rather
/// bad downside however: when Rust cannot control the layout, it cannot make
/// guarantees about the alignment of data within the struct. While x86(_64) and
/// ARM are nice about letting you access memory at arbitrary alignments, most
/// processors are not so nice and will just hard-interrupt if you try.
///
/// Thus, Rust cannot promise that types stored in such a struct will behave the
/// same as the same type stored outside of such a struct on all architectures.
/// To ensure that appropriate masking and shifting always happens, Rust enforces
/// rules around access to members of packed structs that disable most any access
/// other than copies out of that memory.
///
/// This is easy to work around: we just need to add an accessor for all members
/// of a packed struct. That's a ton of boilerplate though; hence this crate.
///
/// ## Example
/// ```rust
/// use anyhow::Result;
/// use nitrogen_packed_struct::packed_struct;
/// use std::mem::{align_of, size_of};
/// use zerocopy::IntoBytes;
///
/// #[packed_struct]
/// #[derive(Copy, Clone)]
/// struct TestStructDerive {
///     a: u8,
///     b: u32,
///     c: u16,
/// }
///
/// #[packed_struct]
/// #[derive(Copy, Clone)]
/// struct TestStructEmpty {
/// }
///
/// fn it_can_derive() -> Result<()> {
///     let ts = TestStructDerive {
///         a: 42,
///         b: 1,
///         c: 256,
///     };
///
///     assert_eq!(size_of::<TestStructDerive>(), 7);
///     assert_eq!(align_of::<TestStructDerive>(), 1);
///
///     // Access members without warning.
///     assert_eq!(ts.a(), 42u8);
///     assert_eq!(ts.b(), 1u32);
///     assert_eq!(ts.c(), 256u16);
///
///     // Make sure Debug derive comes through.
///     println!("Debug output: {ts:#?}");
///
///     // Make sure copy and clone attrs come through.
///     let ts2 = ts;
///     assert_eq!(ts2.a(), 42u8);
///     assert_eq!(ts2.b(), 1u32);
///     assert_eq!(ts2.c(), 256u16);
///
///     // Overlay should work as well
///     let buf: &[u8] = &[42, 1, 0, 0, 0, 0, 1];
///     let ts3 = TestStructDerive::overlay(buf)?;
///     assert_eq!(ts3.a(), 42u8);
///     assert_eq!(ts3.b(), 1u32);
///     assert_eq!(ts3.c(), 256u16);
///
///     // Should round-trip
///     assert_eq!(buf, ts3.as_bytes());
///
///     // Overlaying a prefix should work as well, but not round-trip
///     let buf4: &[u8] = &[42, 1, 0, 0, 0, 0, 1, 42, 42, 42, 42, 42];
///     let ts4 = TestStructDerive::overlay_prefix(buf4)?;
///     assert_eq!(ts4.a(), 42u8);
///     assert_eq!(ts4.b(), 1u32);
///     assert_eq!(ts4.c(), 256u16);
///
///     // Should round-trip
///     assert_ne!(buf4, ts4.as_bytes());
///
///     // Overlay a slice and check the second member
///     let buf5: &[u8] = &[42, 1, 0, 0, 0, 0, 1, 42, 2, 0, 0, 0, 0, 2];
///     let ts5 = TestStructDerive::overlay_slice(buf5)?;
///     assert_eq!(ts5[1].a(), 42u8);
///     assert_eq!(ts5[1].b(), 2u32);
///     assert_eq!(ts5[1].c(), 512u16);
///
///     Ok(())
/// }
///
/// it_can_derive().unwrap();
/// ```
pub use injector::packed_struct;

// Re-export injector deps
pub use anyhow;
pub use zerocopy;
