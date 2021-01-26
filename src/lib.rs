#![no_std]
#![deny(
    // missing_docs,
    rust_2018_idioms, broken_intra_doc_links, private_intra_doc_links,
)]

#[cfg(feature = "example_generated")]
pub mod example_generated;

pub mod cpu;
pub mod mmio;

mod macros;

use core::ops::{BitAnd, BitOr, Not, Shl, Shr};

/// Represents any type that can be used as
/// the underlying value for a register or bitfield.
pub trait Int:
    Clone
    + Copy
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
    + Default
    + sealed::Sealed
{
}

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

/// Obtain the bits that are in the inclusive range of `(start, end)`.
///
/// Note that this mehtod **does not** validate anything,
/// for example the range is out of bounds. It will fail silently and
/// may cause "undefined behaviour" if the wrong arguments are passed.
///
/// # Example
///
/// ```
/// # use rumio::get_bits;
///
/// let x = 0b011011u32;
///
/// assert_eq!(get_bits(x, (1, 3)), 0b101);
/// assert_eq!(get_bits(x, (0, 1)), 0b11);
/// assert_eq!(get_bits(x, (4, 6)), 0b001);
/// ```
pub fn get_bits<I: Int>(num: I, (start, end): (usize, usize)) -> I {
    let bit_len = core::mem::size_of::<I>() * 8;

    // add `1` because this is an inclusive range.
    let end = end + 1;

    let bits = num << (bit_len - end) >> (bit_len - end);
    bits >> start
}

/// Sets the range (inclusive) of bits, given by the `(start, end)` tuple, to the
/// given `bits` value.
///
/// Note that this mehtod **does not** validate anything,
/// for example the range is out of bounds. It will fail silently and
/// may cause "undefined behaviour" if the wrong arguments are passed.
///
/// # Example
///
/// ```
/// # use rumio::set_bits;
///
/// let x = 0u32;
///
/// let x = set_bits(x, (0, 1), 0b11);
/// assert_eq!(x, 0b11);
///
/// let x = set_bits(x, (1, 3), 0b010);
/// assert_eq!(x, 0b0101);
///
/// let x = set_bits(x, (0, 4), 0b11001);
/// assert_eq!(x, 0b11001);
/// ```
pub fn set_bits<I: Int>(num: I, (start, end): (usize, usize), bits: I) -> I {
    let bit_len = core::mem::size_of::<I>() * 8;

    // add `1` because this is an inclusive range.
    let end = end + 1;

    let mask = !I::default() << (bit_len - end) >> (bit_len - end);
    let mask = !(mask >> start << start);

    (num & mask) | (bits << start)
}

#[cfg(test)]
mod tests {
    crate::define_mmio_register! {
        /// Documentation for the `Test` register.
        Test: u64 {
            rw MODE: 0..1 = enum Mode [
                A = 0b00,
                B = 0b01,
                C = 0b10,
                D = 0b11,
            ],

            r FOO: 2,

            rw BAR: 3,
            rw BAZ: 4,

            rw FLAGS: 5..8 = flags Flags [
                A = 0b0001,
                B = 0b0010,
                C = 0b0100,
                D = 0b1000,
            ],
        }
    }
}
