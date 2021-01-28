//#![no_std]
#![deny(
    // missing_docs,
    clippy::all, rust_2018_idioms, broken_intra_doc_links, private_intra_doc_links,
)]

#[cfg(feature = "example_generated")]
pub mod example_generated;

pub mod cpu;
pub mod mmio;
pub mod perm;

mod macros;

use core::{
    marker::PhantomData,
    ops::{BitAnd, BitOr, Not, Shl, Shr},
};
use perm::Permission;

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

/// This macro includes generation of `Int` implementation
/// and generates a `Value::new` method for each type, to be able
/// to make the `new` method const.
macro_rules! impl_int {
    ($($num:ty),*) => {
        $(impl Value<$num> {
            /// Create a new [`Value`] with the given mask and bits.
            pub const fn new(mask: $num, bits: $num) -> Self {
                Self {
                    mask,
                    bits: (bits & mask),
                }
            }
        }
        impl<P> Field<$num, P> {
            /// Create a new [`Field`] that covers the given mask.
            pub const fn new(mask: $num) -> Self {
                Self {
                    mask,
                    __perm: PhantomData,
                }
            }
        }
        impl Int for $num {}
        )*
    };
}

impl_int!(u8, u16, u32, u64);

/// A value that can be applied to any register using
/// the `modify` method.
///
/// This is also used to modify mulitple bitfields in one write operation.
#[derive(Clone, Copy, Debug)]
pub struct Value<I> {
    mask: I,
    bits: I,
}

impl<I: Int> Value<I> {
    /// Modify all bits that are specified by this [`Value`] in
    /// the given value and return the modified version.
    #[inline]
    pub fn modify(self, val: I) -> I {
        (val & !self.mask) | self.bits
    }
}

impl<I: Int> BitOr<Value<I>> for Value<I> {
    type Output = Value<I>;

    fn bitor(self, rhs: Value<I>) -> Self::Output {
        Self {
            mask: self.mask | rhs.mask,
            bits: self.bits | rhs.bits,
        }
    }
}

/// Specifies a specific bit mask inside a register.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Field<I, P> {
    mask: I,
    __perm: PhantomData<P>,
}

impl<I: Int, P: Permission> Field<I, P> {
    /// Return all bits that were covered by this field.
    ///
    /// # Example
    ///
    /// ```
    /// # use rumio::{perm, Field};
    /// # fn main() {
    /// let field = Field::<u32, perm::ReadWrite>::new(0b11110);
    /// let x = 0b10111u32;
    /// assert_eq!(field.read(x), 0b10110);
    /// # }
    /// ```
    pub fn read(self, val: I) -> I {
        val & self.mask
    }
}

impl<I, P1, P2> BitOr<Field<I, P2>> for Field<I, P1>
where
    I: Int,
    P1: perm::Compatible<P1, P2>,
    P2: Permission,
{
    type Output = Field<I, <P1 as perm::Compatible<P1, P2>>::Output>;

    fn bitor(self, rhs: Field<I, P2>) -> Self::Output {
        Field {
            mask: self.mask | rhs.mask,
            __perm: PhantomData,
        }
    }
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

    (num & mask) | ((bits << start) & !mask)
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}
