/// Define abstractions for a CPU register.
///
/// The first line must contain the register struct, that either
/// implements [`RegisterRead`][rr] or [`RegisterWrite`][rw] or both,
/// and the underlying type for this register.
/// After the first line, a comma-separated list of field declaration follows.
///
/// This macro generates a new module, with the given name, for each field.
/// To access a fields data, call `FIELD_NAME::get()` where `FIELD_NAME`
/// comes from the declaration inside the macro. To modify the data,
/// call `FIELD_NAME::set()`.
///
/// he arguments and return values of these methods are determined by the type of this field.
/// Each field can be one of the following types:
///
/// ## Single Bit
///
/// A single bit at a specific position.
/// ```ignore
/// rw IS_ENABLED: 0,
/// ```
///
/// The `rw` represents the permission, and can be either `r`, `w` or `rw`.
/// For example, If the permission is `r`, the bit can only be read, but never
/// written to. After the permission comes the name of the field.
///
/// The `0` specifies the bit inside the register for this field.
///
/// The generated `get()` method returns a bool, indicating `1` or `0`.
/// The set method takes a `bool`.
///
/// ## Enum
///
/// Multiple bits, that together represent a variant of an enum.
/// ```ignore
/// rw MODE: 1..3 = enum Mode [
///     User = 0b00,
///     Supervisor = 0b01,
///     Machine = 0b01,
/// ],
/// ```
///
/// The generated `get` method returns an `Option<Mode>`, which is `None`
/// if the bit pattern is not a valid variant, otherwise it returns
/// the variant with the bit pattern.
///
/// The generated `set` method takes the `Mode` enum and writes the bit pattern
/// of the given variant into the bit range.
///
/// **Note** that the ranges are **inclusive** and **must** be valid.
/// There are no checks at all and providing an invalid range
/// may lead to undefined behaviour. This is also true for the
/// next type, the bitflags.
///
/// ## Bitflags
///
/// Multiple bits, each representing a flag.
/// ```ignore
/// rw FLAGS: 4..7 = flags Flags [
///     A = 0b0001,
///     B = 0b0010,
///     C = 0b0100,
/// ],
/// ```
///
/// The generated `Flags` struct is generated using the [`bitflags`][bf] crate and thus
/// has the exact same API.
///
/// The generated `get` method creates the struct using the `from_bits_truncate` method.
///
///
/// # Example
///
/// ```no_run
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
///
/// rumio::define_cpu_register! { CpuRegister as u64 =>
///     r ENABLED: 0,
///     rw MODE: 1..2 = enum Mode [
///         Sending = 0b00,
///         Receiving = 0b01,
///     ],
///
///     rw FLAGS: 3..6 = flags Flags [
///         A = 0b0001,
///         B = 0b0010,
///         C = 0b0100,
///         D = 0b1000,
///     ],
/// }
///
/// # fn send_data() {}
///
/// fn main() {
///     if !ENABLED::get() {
///         panic!("CPU not enabled");
///     }
///
///     match MODE::get().unwrap() {
///         Mode::Sending => {
///             println!("sending data...");
///             send_data();
///             MODE::set(Mode::Receiving);
///             println!("ready to receive data...");
///         }
///         Mode::Receiving => println!("receiving data..."),
///     }
///
///     let flags = FLAGS::get();
///     if !flags.contains(Flags::A) {
///         println!("A flag is not enabled. Enable it now...");
///         FLAGS::set(flags | Flags::A);
///     }
/// }
/// ```
///
///
/// [rr]: crate::cpu::RegisterRead
/// [rw]: crate::cpu::RegisterWrite
/// [bf]: https://docs.rs/bitflags
#[macro_export]
macro_rules! define_cpu_register {
    ($register:ident as $num_ty:ty => $(
     $(#[$field_attr:meta])*
     $perm:ident $name:ident: $from:literal $( .. $to:literal =
         $(#[$kind_attr:meta])*
         $kind_type:ident $kind_name:ident [
             $(
                 $(#[$kind_variant_attr:meta])*
                 $kind_variant:ident = $kind_variant_val:expr
             ),*$(,)?
         ]
     )?
    ),*$(,)?) => {
        const _: fn() = || {
            fn assert_impl<T: $crate::Int>() {}
            assert_impl::<$num_ty>();
        };

        $($(
            $crate::__generate_field_kinds__!($num_ty, $perm, $from .. $to,
                $(#[$kind_attr])*
                $kind_type $kind_name [
                    $(
                        $(#[$kind_variant_attr])*
                        $kind_variant = $kind_variant_val
                    ),*
                ]
            );
        )*)?

        $(
            $(#[$field_attr])*
            #[allow(non_snake_case, dead_code)]
            pub mod $name {
                $crate::define_cpu_register!(@internal, $num_ty, $register, $perm $name: $from $(.. $to =
                    $kind_type $kind_name [
                        $($kind_variant = $kind_variant_val),*
                    ]
                )?);
            }
        )*

        $crate::__generate_if_perm__! { @read
            /// Get the raw value out of this CPU register.
            pub fn get() -> $num_ty {
                <$register as $crate::cpu::RegisterRead<$num_ty>>::read()
            }
            => $($perm) *
        }

        $crate::__generate_if_perm__! { @read
            /// Read the given field from this register.
            pub fn read<P: $crate::perm::Readable>(field: $crate::Field<$num_ty, P>) -> $num_ty {
                let val = <$register as $crate::cpu::RegisterRead<$num_ty>>::read();
                $crate::Field::<$num_ty, P>::read(field, val)
            }
            => $($perm) *
        }

        $crate::__generate_if_perm__! { @read
            /// Check if one of the given fields is set.
            ///
            /// Returns `true` if the value specified by the field is not null.
            pub fn is_set<P: $crate::perm::Readable>(field: $crate::Field<$num_ty, P>) -> ::core::primitive::bool {
                let val = <$register as $crate::cpu::RegisterRead<$num_ty>>::read();
                $crate::Field::<$num_ty, P>::read(field, val) != 0
            }
            => $($perm) *
        }

        $crate::__generate_if_perm__! { @write
            /// Write the raw value into this CPU register.
            pub fn set(val: $num_ty) {
                <$register as $crate::cpu::RegisterWrite<$num_ty>>::write(val);
            }
            => $($perm) *
        }

        $crate::__generate_if_perm__! { @write
            /// Write the given values into this register and set all other bits to 0.
            pub fn write(val: $crate::Value<$num_ty>) {
                let val = $crate::Value::<$num_ty>::modify(val, 0);
                <$register as $crate::cpu::RegisterWrite<$num_ty>>::write(val);
            }
            => $($perm) *
        }

        $crate::__generate_if_perm__! { @read_write
            /// Modify this register to match the given value, but keep all other bits untouched.
            pub fn modify(val: $crate::Value<$num_ty>) {
                let reg = <$register as $crate::cpu::RegisterRead<$num_ty>>::read();
                let reg = $crate::Value::<$num_ty>::modify(val, reg);
                <$register as $crate::cpu::RegisterWrite<$num_ty>>::write(reg);
            }
            => $($perm) *
        }
    };

    // =====================================
    // Read and write bitflags
    // =====================================

    (@internal, $num_ty:ty, $register:ident, rw $name:ident: $from:literal .. $to:literal = flags $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        $crate::define_cpu_register!(@internal, $num_ty, $register, r $name: $from .. $to = flags $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);

        $crate::define_cpu_register!(@internal, $num_ty, $register, w $name: $from .. $to = flags $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);
    };

    (@internal, $num_ty:ty, $register:ident, r $name:ident: $from:literal .. $to:literal = flags $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        /// Read the raw bits from the register and return a struct representing
        /// all flags of this bit range.
        pub fn get() -> super::$kind_name {
            let val = <super::$register as $crate::cpu::RegisterRead<$num_ty>>::read();
            super::$kind_name::from_bits_truncate($crate::get_bits(val, ($from, $to)))
        }
    };

    (@internal, $num_ty:ty, $register:ident, w $name:ident: $from:literal .. $to:literal = flags $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        /// Set this bit range to the given bitflags.
        pub fn set(flags: super::$kind_name) {
            let bits = super::$kind_name::bits(&flags);
            let val = <super::$register as $crate::cpu::RegisterRead<$num_ty>>::read();
            let val = $crate::set_bits(val, ($from, $to), bits);
            <super::$register as $crate::cpu::RegisterWrite<$num_ty>>::write(val);
        }
    };

    // =====================================
    // Read and write a enum range of bits
    // =====================================

    (@internal, $num_ty:ty, $register:ident, rw $name:ident: $from:literal .. $to:literal = enum $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        $crate::define_cpu_register!(@internal, $num_ty, $register, r $name: $from .. $to = enum $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);

        $crate::define_cpu_register!(@internal, $num_ty, $register, w $name: $from .. $to = enum $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);
    };

    (@internal, $num_ty:ty, $register:ident, r $name:ident: $from:literal .. $to:literal = enum $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        /// Read the raw bits from the register, and then try to map them to an enum.
        pub fn get() -> ::core::option::Option<super::$kind_name> {
            let val = <super::$register as $crate::cpu::RegisterRead<$num_ty>>::read();
            match $crate::get_bits(val, ($from, $to)) {
                $($kind_variant_val => ::core::option::Option::Some(super::$kind_name::$kind_variant),)*
                _ => ::core::option::Option::None,
            }
        }
    };

    (@internal, $num_ty:ty, $register:ident, w $name:ident: $from:literal .. $to:literal = enum $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        /// Set this bits to the given value.
        pub fn set(val: super::$kind_name) {
            let bits = match val {
                $(super::$kind_name::$kind_variant => $kind_variant_val,)*
            };
            let val = <super::$register as $crate::cpu::RegisterRead<$num_ty>>::read();
            let val = $crate::set_bits(val, ($from, $to), bits);
            <super::$register as $crate::cpu::RegisterWrite<$num_ty>>::write(val);
        }
    };

    // =====================================
    // Read and write a single bit
    // =====================================

    (@internal, $num_ty:ty, $register:ident, $perm:ident $name:ident: $bit:literal) => {
        /// A `Field` that covers this single bit.
        pub const FIELD: $crate::Field<$num_ty, $crate::__perm_for_name__!($perm)> = $crate::Field::<$num_ty, _>::new(1 << $bit);

        $crate::define_cpu_register!(@internal_bit, $num_ty, $register, $perm $name: $bit);
    };

    (@internal_bit, $num_ty:ty, $register:ident, rw $name:ident: $bit:literal) => {
        $crate::define_cpu_register!(@internal_bit, $num_ty, $register, r $name: $bit);
        $crate::define_cpu_register!(@internal_bit, $num_ty, $register, w $name: $bit);
    };

    (@internal_bit, $num_ty:ty, $register:ident, r $name:ident: $bit:literal) => {
        /// Check if this bit is set inside the CPU register.
        pub fn get() -> ::core::primitive::bool {
            let val = <super::$register as $crate::cpu::RegisterRead<$num_ty>>::read();
            val & (1 << $bit) != 0
        }
    };

    (@internal_bit, $num_ty:ty, $register:ident, w $name:ident: $bit:literal) => {
        /// A `Value` that will set this bit to high when modifying a register.
        pub const SET: $crate::Value<$num_ty> = $crate::Value::<$num_ty>::new(1 << $bit, 1 << $bit);

        /// A `Value` that will set this bit to low when modifying a register.
        pub const CLEAR: $crate::Value<$num_ty> = $crate::Value::<$num_ty>::new(1 << $bit, 0);

        /// Set the value of this bit inside the CPU register.
        pub fn set(x: ::core::primitive::bool) {
            const MASK: $num_ty = 1 << $bit;
            match x {
                true => <super::$register as $crate::cpu::RegisterWrite<$num_ty>>::set(MASK),
                false => <super::$register as $crate::cpu::RegisterWrite<$num_ty>>::clear(MASK),
            }
        }
    };
}

/// Provide a simple implementation for the [`RegisterWrite::set()`](super::RegisterWrite::clear) method.
///
/// Put this macro into your [`set`](super::RegisterWrite::set) implementation for
/// [`RegisterWrite`](super::RegisterWrite).
/// This macro only works if the register implements [`RegisterRead`](super::RegisterRead),
/// because it will first read the value, set the bits, and write the value to this register.
///
/// The same can be done for [`clear`](super::RegisterWrite::clear) using the [`impl_cpu_clear`] macro.
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

/// Provide a simple implementation for the [`RegisterWrite::clear()`](super::RegisterWrite::clear) method.
///
/// Put this macro into your [`clear`](super::RegisterWrite::clear) implementation for [`RegisterWrite`](super::RegisterWrite).
/// This macro only works if the register implements [`RegisterRead`](super::RegisterRead),
/// because it will first read the value, clear the bits, and write the value to this register.
///
/// The same can be done for [`set`](super::RegisterWrite::set) using the [`impl_cpu_set`] macro.
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
