#[macro_export]
macro_rules! define_mmio_register {
    ($(#[$reg_attr:meta])*
     $reg_name:ident: $num_ty:ty { $(
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
    ),*$(,)?
    }) => {
        const _: fn() = || {
            fn assert_impl<T: $crate::Int>() {}
            assert_impl::<$num_ty>();
        };

        $($(
            $crate::__generate_field_kinds__!($num_ty,
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
            #[derive(Clone, Copy)]
            pub struct $name($crate::mmio::VolAddr<$num_ty>);
        )*

        $(#[$reg_attr])*
        #[derive(Clone, Copy)]
        pub struct $reg_name($crate::mmio::VolAddr<$num_ty>);

        #[allow(dead_code)]
        impl $reg_name {
            /// Perform a volatile read and return the raw valuue of this register.
            #[inline]
            pub fn read(&self) -> $num_ty {
                $crate::mmio::VolAddr::read(self.0)
            }

            /// Write the raw value into this register using a volatile write.
            #[inline]
            pub fn write(&self, val: $num_ty) {
                $crate::mmio::VolAddr::write(self.0, val)
            }

            $(#[allow(non_snake_case)]
            $(#[$field_attr])*
            pub fn $name(&self) -> $name {
                $name(self.0)
            })*
        }

        $(
            $crate::define_mmio_register!(@internal, $num_ty, $perm $name: $from $(.. $to =
                $kind_type $kind_name [
                    $($kind_variant = $kind_variant_val),*
                ]
            )?);
        )*
    };

    // =====================================
    // Read and write bitflags
    // =====================================

    (@internal, $num_ty:ty, rw $name:ident: $from:literal .. $to:literal = flags $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        $crate::define_mmio_register!(@internal, $num_ty, r $name: $from .. $to = flags $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);

        $crate::define_mmio_register!(@internal, $num_ty, w $name: $from .. $to = flags $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);
    };

    (@internal, $num_ty:ty, r $name:ident: $from:literal .. $to:literal = flags $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        impl $name {
            /// Read the raw bits from the register and return a struct representing
            /// all flags of this bit range.
            #[allow(unused)]
            pub fn get(&self) -> $kind_name {
                let val = $crate::mmio::VolAddr::read(self.0);
                $kind_name::from_bits_truncate($crate::get_bits(val, ($from, $to)))
            }
        }
    };

    (@internal, $num_ty:ty, w $name:ident: $from:literal .. $to:literal = flags $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        impl $name {
            /// Set this bit range to the given bitflags.
            #[allow(unused)]
            pub fn set(&self, flags: $kind_name) {
                let bits = $kind_name::bits(&flags);
                let val = $crate::mmio::VolAddr::read(self.0);
                $crate::mmio::VolAddr::write(self.0, $crate::set_bits(val, ($from, $to), bits));
            }
        }
    };

    // =====================================
    // Read and write a enum range of bits
    // =====================================

    (@internal, $num_ty:ty, rw $name:ident: $from:literal .. $to:literal = enum $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        $crate::define_mmio_register!(@internal, $num_ty, r $name: $from .. $to = enum $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);

        $crate::define_mmio_register!(@internal, $num_ty, w $name: $from .. $to = enum $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);
    };

    (@internal, $num_ty:ty, r $name:ident: $from:literal .. $to:literal = enum $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        impl $name {
            /// Read the raw bits from the register, and then try to map them to an enum.
            #[allow(unused)]
            pub fn get(&self) -> ::core::option::Option<$kind_name> {
                let val = $crate::mmio::VolAddr::read(self.0);
                match $crate::get_bits(val, ($from, $to)) {
                    $($kind_variant_val => ::core::option::Option::Some($kind_name::$kind_variant),)*
                    _ => ::core::option::Option::None,
                }
            }
        }
    };

    (@internal, $num_ty:ty, w $name:ident: $from:literal .. $to:literal = enum $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        impl $name {
            /// Set this bits to the given value.
            #[allow(unused)]
            pub fn set(&self, val: $kind_name) {
                let bits = match val {
                    $($kind_name::$kind_variant => $kind_variant_val,)*
                };
                let val = $crate::mmio::VolAddr::read(self.0);
                let val = $crate::set_bits(val, ($from, $to), bits);
                $crate::mmio::VolAddr::write(self.0, val);
            }
        }
    };

    // =====================================
    // Read and write a single bit
    // =====================================

    (@internal, $num_ty:ty, rw $name:ident: $bit:literal) => {
        $crate::define_mmio_register!(@internal, $num_ty, r $name: $bit);
        $crate::define_mmio_register!(@internal, $num_ty, w $name: $bit);
    };

    (@internal, $num_ty:ty, r $name:ident: $bit:literal) => {
        impl $name {
            /// Check if this bit is set inside the MMIO.
            #[allow(unused)]
            pub fn get(&self) -> ::core::primitive::bool {
                let val = $crate::mmio::VolAddr::read(self.0);
                val & (1 << $bit) != 0
            }
        }
    };

    (@internal, $num_ty:ty, w $name:ident: $bit:literal) => {
        impl $name {
            /// Set the value of this bit inside the MMIO.
            #[allow(unused)]
            pub fn set(&self, x: ::core::primitive::bool) {
                const MASK: $num_ty = 1 << $bit;
                let val = $crate::mmio::VolAddr::read(self.0);
                let val = match x {
                    true => val | MASK,
                    false=> val & !MASK,
                };
                $crate::mmio::VolAddr::write(self.0, val);
            }
        }
    };
}
