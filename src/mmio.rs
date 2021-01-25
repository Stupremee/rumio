//! Abstractions for MMIO regions.

mod macros;

use core::{marker::PhantomData, num::NonZeroUsize};

/// An address that can only be accessed by volatile reads and writes.
///
/// Note that this structure does not guarantee any synchronization
/// and will only ensure that volatile reads/writes are used.
///
/// # Safety
///
/// - The address must be [valid][valid] as defined by the [`core::ptr`] rules.
/// - The address must be non-zero and must be aligned to `T`.
/// - The underlying memory must be a valid bit-pattern for `T`.
///
///
/// Most of this is a clone of [`voladdress`](https://docs.rs/voladdress) to allow
/// more customization and avoid the dependency.
///
/// [valid]: https://doc.rust-lang.org/core/ptr/index.html#safety
pub struct VolAddr<T> {
    addr: NonZeroUsize,
    _type: PhantomData<*mut T>,
}

impl<T> VolAddr<T> {
    /// Create a new [`VolAddr`] at the given address.
    ///
    /// # Safety
    ///
    /// This method must follow the safety arguments of this type.
    pub const unsafe fn new(addr: usize) -> Self {
        Self {
            addr: NonZeroUsize::new_unchecked(addr),
            _type: PhantomData,
        }
    }

    /// Cast this [`VolAddr`] to a new type.
    ///
    /// # Safety
    ///
    /// This method must follow the safety arguments of this type.
    pub const unsafe fn cast<U>(self) -> VolAddr<U> {
        VolAddr {
            addr: self.addr,
            _type: PhantomData,
        }
    }

    /// Offset this address by the given `offset`.
    ///
    /// This method will wrap around on an overflow.
    ///
    /// # Safety
    ///
    /// This method must follow the safety arguments of this type.
    pub const unsafe fn offset(self, offset: isize) -> Self {
        Self {
            addr: NonZeroUsize::new_unchecked(
                self.addr
                    .get()
                    .wrapping_add(offset as usize * core::mem::size_of::<T>()),
            ),
            _type: PhantomData,
        }
    }

    /// Perfoms a volatile read of this address, and returns a copy of the inner `T`.
    ///
    /// This method is safe, because all safety guarantees must be provided
    /// when creating a new [`VolAddr`], and the [`Copy`] bound prevents the returning value
    /// from running code in the [`Drop`] implementation.
    pub fn read(self) -> T
    where
        T: Copy,
    {
        unsafe { core::ptr::read_volatile(self.addr.get() as *mut T) }
    }

    /// Performs a volatile write to this address using the given value.
    ///
    /// This method requires `T` to implement [`Copy`], because the [`Drop`]
    /// implementation would never be run if it's written to this address.
    pub fn write(self, val: T)
    where
        T: Copy,
    {
        unsafe { core::ptr::write_volatile(self.addr.get() as *mut T, val) }
    }
}
