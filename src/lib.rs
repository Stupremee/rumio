#![forbid(unsafe_code)]
#![deny(
    // missing_docs,
    rust_2018_idioms, clippy::pedantic,
    broken_intra_doc_links, private_intra_doc_links,
)]

pub mod cpu;

mod macros;

/// Represents any type that can be used as
/// the underlying value for a register or bitfield.
pub trait Int: Clone + Copy + sealed::Sealed {}

impl Int for u8 {}
impl Int for u16 {}
impl Int for u32 {}
impl Int for u64 {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}
