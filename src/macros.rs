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
            $crate::define_cpu_register!(@internal_gen_kind, $num_ty,
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

    (@internal, $num_ty:ty, $register:ident, rw $name:ident: $bit:literal) => {
        $crate::define_cpu_register!(@internal, $num_ty, $register, r $name: $bit);
        $crate::define_cpu_register!(@internal, $num_ty, $register, w $name: $bit);
    };

    (@internal, $num_ty:ty, $register:ident, r $name:ident: $bit:literal) => {
        /// Check if this bit is set inside the CPU register.
        pub fn get() -> ::core::primitive::bool {
            let val = <super::$register as $crate::cpu::RegisterRead<$num_ty>>::read();
            val & (1 << $bit) != 0
        }
    };

    (@internal, $num_ty:ty, $register:ident, w $name:ident: $bit:literal) => {
        /// Set the value of this inside the CPU register.
        pub fn set(x: ::core::primitive::bool) {
            const MASK: $num_ty = 1 << $bit;
            match x {
                true => <super::$register as $crate::cpu::RegisterWrite<$num_ty>>::set(MASK),
                false => <super::$register as $crate::cpu::RegisterWrite<$num_ty>>::clear(MASK),
            }
        }
    };

    // =====================================
    // Generate the kind enums and bitflags
    // =====================================

    (@internal_gen_kind, $num_ty:ty,
        $(#[$attr:meta])*
        enum $kind_name:ident [$(
            $(#[$variant_attr:meta])*
            $variant:ident = $variant_val:expr
        ),*]
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $kind_name {
            $( $(#[$variant_attr])* $variant ),*
        }
    };

    (@internal_gen_kind, $num_ty:ty,
        $(#[$attr:meta])*
        flags $kind_name:ident [$(
            $(#[$variant_attr:meta])*
            $variant:ident = $variant_val:expr
        ),*]
    ) => {
        ::bitflags::bitflags! {
            $(#[$attr])*
            pub struct $kind_name: $num_ty {
                $(const $variant = $variant_val;)*
            }
        }
    };
}
