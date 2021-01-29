//! Abstractions for MMIO regions.

mod macros;

use core::{fmt, marker::PhantomData, num::NonZeroUsize};

/// A structure that represents any type, and can be used
/// to have any type inside a MMIO struct.
///
/// # Example
///
/// ```
/// # use rumio::mmio::Lit;
/// rumio::define_mmio_struct! {
///     pub struct Device {
///         0x00 => one: Lit<u64>,
///         0x08 => two: Lit<u8>,
///     }
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lit<T>(VolAddr<T>);

impl<T> Lit<T> {
    /// Create a new `Lit` at the given address.
    pub fn new(addr: VolAddr<T>) -> Self {
        Self(addr)
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
        self.0.read()
    }

    /// Perfoms a volatile read of this address, and returns the inner `T`.
    ///
    /// # Safety
    ///
    /// This method doesn't require the `Copy` bound for `T`, and thus the caller
    /// must make sure that dropping the returned value multiple times doesn't cause UB.
    #[inline]
    pub unsafe fn read_non_copy(self) -> T {
        self.0.read_non_copy()
    }

    /// Performs a volatile write to this address using the given value.
    ///
    /// Note that the `Drop` implementation of `T` will never be run.
    #[inline]
    pub fn write(self, val: T) {
        self.0.write(val);
    }
}

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
#[repr(transparent)]
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
    #[inline]
    pub fn read(self) -> T
    where
        T: Copy,
    {
        unsafe { core::ptr::read_volatile(self.addr.get() as *mut T) }
    }

    /// Perfoms a volatile read of this address, and returns the inner `T`.
    ///
    /// # Safety
    ///
    /// This method doesn't require the `Copy` bound for `T`, and thus the caller
    /// must make sure that dropping the returned value multiple times doesn't cause UB.
    #[inline]
    pub unsafe fn read_non_copy(self) -> T {
        core::ptr::read_volatile(self.addr.get() as *mut T)
    }

    /// Performs a volatile write to this address using the given value.
    ///
    /// Note that the `Drop` implementation of `T` will never be run.
    #[inline]
    pub fn write(self, val: T) {
        unsafe { core::ptr::write_volatile(self.addr.get() as *mut T, val) }
    }
}

impl<T> Clone for VolAddr<T> {
    fn clone(&self) -> Self {
        Self {
            addr: self.addr,
            _type: PhantomData,
        }
    }
}
impl<T> Copy for VolAddr<T> {}

impl<T> fmt::Debug for VolAddr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VolAddr({:p})", self)
    }
}
impl<T> fmt::Pointer for VolAddr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:p}", self.addr.get() as *mut T)
    }
}

impl<T> PartialEq for VolAddr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl<T> Eq for VolAddr<T> {}

impl<T> PartialOrd for VolAddr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.addr.partial_cmp(&other.addr)
    }
}
impl<T> Ord for VolAddr<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.addr.cmp(&other.addr)
    }
}
