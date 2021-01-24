//! Traits for accessing CPU registers.

/// Trait for read and write functionality of a CPU register.
pub trait RegisterReadWrite {
    /// The value that represents the register.
    type Value: crate::Int;

    /// Read the raw value from this CPU register.
    fn read() -> Self::Value;

    /// Write the given value into this CPU register.
    fn write(val: Self::Value);
}
