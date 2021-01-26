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
            $crate::define_mmio_register!(@internal, $num_ty, $reg_name, $perm $name: $from $(.. $to =
                $kind_type $kind_name [
                    $($kind_variant = $kind_variant_val),*
                ]
            )?);
        )*
    };

    // =====================================
    // Read and write bitflags
    // =====================================

    (@internal, $num_ty:ty, $reg_name:ident, rw $name:ident: $from:literal .. $to:literal = flags $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        $crate::define_mmio_register!(@internal, $num_ty, $reg_name, r $name: $from .. $to = flags $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);

        $crate::define_mmio_register!(@internal, $num_ty, $reg_name, w $name: $from .. $to = flags $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);
    };

    (@internal, $num_ty:ty, $reg_name:ident, r $name:ident: $from:literal .. $to:literal = flags $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
    };

    (@internal, $num_ty:ty, $reg_name:ident, w $name:ident: $from:literal .. $to:literal = flags $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
    };

    // =====================================
    // Read and write a enum range of bits
    // =====================================

    (@internal, $num_ty:ty, $reg_name:ident, rw $name:ident: $from:literal .. $to:literal = enum $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
        $crate::define_mmio_register!(@internal, $num_ty, $reg_name, r $name: $from .. $to = enum $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);

        $crate::define_mmio_register!(@internal, $num_ty, $reg_name, w $name: $from .. $to = enum $kind_name [
            $($kind_variant = $kind_variant_val),*
        ]);
    };

    (@internal, $num_ty:ty, $reg_name:ident, r $name:ident: $from:literal .. $to:literal = enum $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
    };

    (@internal, $num_ty:ty, $reg_name:ident, w $name:ident: $from:literal .. $to:literal = enum $kind_name:ident [
        $($kind_variant:ident = $kind_variant_val:expr),*
    ]) => {
    };

    // =====================================
    // Read and write a single bit
    // =====================================

    (@internal, $num_ty:ty, $reg_name:ident, rw $name:ident: $bit:literal) => {
        $crate::define_mmio_register!(@internal, $num_ty, $reg_name, r $name: $bit);
        $crate::define_mmio_register!(@internal, $num_ty, $reg_name, w $name: $bit);
    };

    (@internal, $num_ty:ty, $reg_name:ident, r $name:ident: $bit:literal) => {
        impl $name {
            /// Check if this bit is set inside the MMIO.
            #[allow(unused)]
            pub fn get(&self) -> ::core::primitive::bool {
                let val = $crate::mmio::VolAddr::read(self.0);
                val & (1 << $bit) != 0
            }
        }
    };

    (@internal, $num_ty:ty, $reg_name:ident, w $name:ident: $bit:literal) => {
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
