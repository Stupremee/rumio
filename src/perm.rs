//! Marker trait and structs for providing type-safe permissions.

use crate::sealed::Sealed;

/// Sealed trait for marking a permission.
pub trait Permission: Sealed {}

/// Sealed trait for marking a permission that can read.
pub trait Readable: Permission {}

/// Sealed trait for marking a permission that can write.
pub trait Writable: Permission {}

/// Represents a read-only permission.
pub enum ReadOnly {}
impl Permission for ReadOnly {}
impl Readable for ReadOnly {}
impl Sealed for ReadOnly {}

impl<P: Readable> CompatibleWith<ReadOnly, P> for ReadOnly {}

/// Represents a write-only permission.
pub enum WriteOnly {}
impl Permission for WriteOnly {}
impl Writable for WriteOnly {}
impl Sealed for WriteOnly {}

impl<P: Writable> CompatibleWith<WriteOnly, P> for WriteOnly {}

/// Represents a read-write permission.
pub enum ReadWrite {}
impl Permission for ReadWrite {}
impl Readable for ReadWrite {}
impl Writable for ReadWrite {}
impl Sealed for ReadWrite {}

impl<P: Permission> CompatibleWith<ReadWrite, P> for ReadWrite {}

/// Marker trait that makes two permission types compatible.
pub trait CompatibleWith<P1, P2>: Sealed {}
