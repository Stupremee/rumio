//! `rumio`
//! =======
//! [![Crates.io](https://img.shields.io/crates/v/rumio.svg)](https://crates.io/crates/rumio)
//! [![Documentation](https://img.shields.io/badge/documentation-docs.rs-blue.svg)](https://docs.rs/rumio)
//!
//! **Control your MMIO and CPU registers without pain.**
//!
//! [Documentation][docs-rs] | [Crate][crates-io] | [Examples][examples]
//!
//! This crate provides various macros to generate a nice API for [MMIO][mmio] blocks
//! and CPU registers. It's mainly meant as a replacement for the [`register`][regs-rs] crate
//! to provide a better API and make the work easier.
//!
//! # Usage
//!
//! For more updated and larger examples take a look at the [tests][examples].
//!
//! ## Defining CPU registers
//!
//! The CPU registers are only useful for control registers which store their data using
//! bitfields. For example the Control-Status-Register of the RISC-V architecture.
//!
//! ```ignore
//! #![feature(asm)]
//!
//! mod mstatus {
//!   use rumio::cpu::{RegisterRead, RegisterWrite};
//!
//!   // first we need to define a register, and a way to read/write to it.
//!   // we will use the `mstatus` CSR from the RISC-V architecture as an example
//!   struct Mstatus;
//!
//!   // the `usize` argument indicates the underyling value of the register.
//!   impl RegisterRead<usize> for Mstatus {
//!       #[inline]
//!       fn read() -> usize {
//!           let val;
//!           unsafe { asm!("csrr {}, mstatus", out(reg) val) }
//!           val
//!       }
//!   }
//!
//!   impl RegisterWrite<usize> for Mstatus {
//!       #[inline]
//!       fn write(val: usize) {
//!           unsafe { asm!("csrw mstatus, {}", in(reg) val) }
//!       }
//!
//!       #[inline]
//!       fn set(mask: usize) {
//!           // `impl_cpu_set` and `impl_cpu_clear` can generated `set` and `clear`
//!           // by performing a read, setting the bits and then write the value again.
//!           rumio::impl_cpu_set!(Self, mask);
//!       }
//!
//!       #[inline]
//!       fn clear(mask: usize) {
//!           rumio::impl_cpu_clear!(Self, mask);
//!       }
//!   }
//!
//!   // now define the different bits and fields of this register
//!   rumio::define_cpu_register! { Mstatus as usize =>
//!     /// Globally enables interrupts in U-Mode.
//!     rw UIE: 0,
//!     /// Globally enables interrupts in S-Mode.
//!     rw SIE: 1,
//!     /// Globally enables interrupts in M-Mode.
//!     rw MIE: 3,
//!
//!     /// The privilege mode a trap in M-Mode was taken from.
//!     r MPP: 11..12 = enum PrivilegeMode [
//!       User = 0b00,
//!       Supervisor = 0b01,
//!       Machine = 0b11,
//!     ],
//!
//!     /// This is not an actual flag of the `mstatus` register, but
//!     /// we add it here for showing the usage of `flags`
//!     rw FLAGS: 13..16 = flags CpuFlags [
//!       A = 0b0001,
//!       B = 0b0010,
//!       C = 0b0100,
//!       D = 0b1000,
//!     ],
//!   }
//! }
//!
//! // the generated api then can be used like this.
//! // to explore the full api generated by this macro, check the `example_generated`
//! // module on docs.rs, and check the examples (the tests are the examples)
//!
//! mstatus::modify(mstatus::UIE::SET | mstatus::SIE::SET | mstatus::MIE::SET);
//! println!("Trap was taken from {:?}", mstatus::MPP::get());
//! ```
//!
//! ## Defining MMIO registers
//!
//! ```ignore
//! // define one MMIO register whose base type is `u16` and name is `Reg`.
//! rumio::define_mmio_register! {
//!     Reg: u16 {
//!         rw MODE: 0..1 = enum Mode [
//!             A = 0b00,
//!             B = 0b01,
//!             C = 0b10,
//!             D = 0b11,
//!         ],
//!
//!         r FOO: 2,
//!
//!         rw BAR: 3,
//!         rw BAZ: 4,
//!
//!         rw FLAGS: 5..8 = flags Flags [
//!             A = 0b0001,
//!             B = 0b0010,
//!             C = 0b0100,
//!             D = 0b1000,
//!         ],
//!     }
//! }
//!
//! rumio::define_mmio_struct! {
//!     pub struct Device {
//!         0x00 => one: Reg,
//!         0x08 => two: Reg,
//!     }
//! }
//!
//! // create a new `Device` at address `0xF00D_BABE
//! let mmio = unsafe { Device::new(0xF00D_BABE) };
//!
//! // access the `one` register
//! let one = mmio.one();
//!
//! // now `one` can be used similarly to the cpu register
//! one.MODE().set(Mode::B);
//! one.FLAGS().set(Flags::B | Flags::C);
//!
//! one.modify(Mode::A | BAR::SET);
//! ```
//!
//! ### License
//!
//! Licensed under either [Apache License][apache] or the [MIT][mit] license.
//!
//!
//! [docs-rs]: https://docs.rs/rumio
//! [crates-io]: https://crates.io/crates/ruumio
//! [examples]: https://github.com/Stupremee/rumio/tree/main/tests
//! [apache]: https://github.com/Stupremee/rumio/tree/main/LICENSE-APACHE
//! [mit]: https://github.com/Stupremee/rumio/tree/main/LICENSE-MIT
//! [mmio]: https://en.wikipedia.org/wiki/Memory-mapped_I/O
//! [regs-rs]: https://docs.rs/register
#![no_std]
#![deny(
    missing_docs,
    clippy::all,
    rust_2018_idioms,
    broken_intra_doc_links,
    private_intra_doc_links
)]

#[cfg(feature = "example_generated")]
pub mod example_generated;

// private re-export for making it available in 
// the macros.

#[doc(hidden)]
pub use defile;
#[doc(hidden)]
pub use bitflags;

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

impl_int!(u8, u16, u32, u64, usize);

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
    impl Sealed for usize {}
}
