//! Traits for accessing CPU registers.

mod macros;

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
