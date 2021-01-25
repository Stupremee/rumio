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
