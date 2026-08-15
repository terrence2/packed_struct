use anyhow::Result;
use nitrogen_packed_struct::packed_struct;
use std::mem::{align_of, size_of};
use zerocopy::IntoBytes;

#[packed_struct]
#[derive(Copy, Clone)]
struct TestStructDerive {
    a: u8,
    b: u32,
    c: u16,
}

#[packed_struct]
#[derive(Copy, Clone)]
struct TestStructEmpty {}

#[test]
fn it_can_derive() -> Result<()> {
    let ts = TestStructDerive {
        a: 42,
        b: 1,
        c: 256,
    };

    assert_eq!(size_of::<TestStructDerive>(), 7);
    assert_eq!(align_of::<TestStructDerive>(), 1);

    // Access members without warning.
    assert_eq!(ts.a(), 42u8);
    assert_eq!(ts.b(), 1u32);
    assert_eq!(ts.c(), 256u16);

    // Make sure Debug derive comes through.
    println!("Debug output: {ts:#?}");

    // Make sure copy and clone attrs come through.
    let ts2 = ts;
    assert_eq!(ts2.a(), 42u8);
    assert_eq!(ts2.b(), 1u32);
    assert_eq!(ts2.c(), 256u16);

    // Overlay should work as well
    let buf: &[u8] = &[42, 1, 0, 0, 0, 0, 1];
    let ts3 = TestStructDerive::overlay(buf)?;
    assert_eq!(ts3.a(), 42u8);
    assert_eq!(ts3.b(), 1u32);
    assert_eq!(ts3.c(), 256u16);

    // Should round-trip
    assert_eq!(buf, ts3.as_bytes());

    // Overlaying a prefix should work as well, but not round-trip
    let buf4: &[u8] = &[42, 1, 0, 0, 0, 0, 1, 42, 42, 42, 42, 42];
    let ts4 = TestStructDerive::overlay_prefix(buf4)?;
    assert_eq!(ts4.a(), 42u8);
    assert_eq!(ts4.b(), 1u32);
    assert_eq!(ts4.c(), 256u16);

    // Should round-trip
    assert_ne!(buf4, ts4.as_bytes());

    // Overlay a slice and check the second member
    let buf5: &[u8] = &[42, 1, 0, 0, 0, 0, 1, 42, 2, 0, 0, 0, 0, 2];
    let ts5 = TestStructDerive::overlay_slice(buf5)?;
    assert_eq!(ts5[1].a(), 42u8);
    assert_eq!(ts5[1].b(), 2u32);
    assert_eq!(ts5[1].c(), 512u16);

    Ok(())
}
