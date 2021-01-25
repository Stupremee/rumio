//! Traits for accessing CPU registers.

/// Trait for reading from a CPU register.
///
pub trait RegisterRead<I: crate::Int> {
    /// Read the raw value from this CPU register.
    ///
    /// It's recommended to implement this method as `#[inline]`
    fn read() -> I;
}

/// Trait for writing into a CPU register.
///
/// It's recommended to implement all of these methods as `#[inline]`.
pub trait RegisterWrite<I: crate::Int> {
    /// Write the given value into this CPU register.
    fn write(val: I);

    /// Set all bits that high in the mask, to `1`
    /// inside this CPU register.
    ///
    /// This can be implemented by reading the value first,
    /// settings the bits and then update the value, if
    /// your architecture doesn't have a bit set instruction.
    fn set(mask: I);

    /// Set all bits that high in the mask, to `0`
    /// inside this CPU register.
    ///
    /// This can be implemented by reading the value first,
    /// clearing the bits and then update the value, if
    /// your architecture doesn't have a bit clear instruction.
    fn clear(mask: I);
}

/// Provide a simple implementation for the [`RegisterWrite::set()`] method.
///
/// Put this macro into your [`set`](RegisterWrite::set) implementation for [`RegisterWrite`].
/// This macro only works if the register implements [`RegisterRead`],
/// because it will first read the value, set the bits, and write the value to this register.
///
/// The same can be done for [`clear`](RegisterWrite::clear) using the [`impl_cpu_clear`] macro.
///
/// # Example
///
/// ```
/// # use rumio::cpu::{RegisterRead, RegisterWrite};
/// pub struct CpuRegister;
///
/// impl RegisterRead<u64> for CpuRegister {
///     fn read() -> u64 {
///         // ...
///         # unimplemented!()
///     }
/// }
///
/// impl RegisterWrite<u64> for CpuRegister {
///     fn write(val: u64) {
///         // ...
///     }
///
///     fn set(mask: u64) {
///         rumio::impl_cpu_set!(Self, mask);
///     }
///
///     fn clear(mask: u64) {
///         rumio::impl_cpu_clear!(Self, mask);
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_cpu_set {
    ($this:ident, $mask:ident) => {
        <$this as $crate::cpu::RegisterWrite<_>>::write(
            <$this as $crate::cpu::RegisterRead<_>>::read() | $mask,
        )
    };
}

/// Provide a simple implementation for the [`RegisterWrite::clear()`] method.
///
/// Put this macro into your [`clear`](RegisterWrite::clear) implementation for [`RegisterWrite`].
/// This macro only works if the register implements [`RegisterRead`],
/// because it will first read the value, clear the bits, and write the value to this register.
///
/// The same can be done for [`set`](RegisterWrite::set) using the [`impl_cpu_set`] macro.
///
/// # Example
///
/// ```
/// # use rumio::cpu::{RegisterRead, RegisterWrite};
/// pub struct CpuRegister;
///
/// impl RegisterRead<u64> for CpuRegister {
///     fn read() -> u64 {
///         // ...
///         # unimplemented!()
///     }
/// }
///
/// impl RegisterWrite<u64> for CpuRegister {
///     fn write(val: u64) {
///         // ...
///     }
///
///     fn set(mask: u64) {
///         rumio::impl_cpu_set!(Self, mask);
///     }
///
///     fn clear(mask: u64) {
///         rumio::impl_cpu_clear!(Self, mask);
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_cpu_clear {
    ($this:ident, $mask:ident) => {
        <$this as $crate::cpu::RegisterWrite<_>>::write(
            <$this as $crate::cpu::RegisterRead<_>>::read() & !$mask,
        )
    };
}
