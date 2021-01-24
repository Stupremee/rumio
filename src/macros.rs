#[macro_export]
macro_rules! define_cpu_register {
    ($register:ident as $num_ty:ty => $(
     $(#[$field_attr:meta])*
     $perm:ident $name:ident: $from:literal $( .. $to:literal =
         $(#[$kind_attr:meta])*
         $($bitflags:ident)? $kind_name:ident [
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
            $(#[$kind_attr])*
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub enum $kind_name {
                $(
                    $(#[$kind_variant_attr])*
                    $kind_variant
                ),*
            }
        )*)?

        $(
            $(#[$field_attr])*
            #[allow(non_snake_case)]
            pub mod $name {
                $crate::define_cpu_register!(@internal, $register, $perm $name: $from $(.. $to =
                    $($bitflags)? $kind_name [
                        $($kind_variant = $kind_variant_val),*
                    ]
                )?);
            }
        )*
    };

    (@internal, $register:ident, r $name:ident: $bit:literal) => {
        /// Read the raw value from this bit inside the bitfield.
        pub fn get() -> bool {
            let val = <super::$register as $crate::cpu::RegisterReadWrite>::read();
            val & (1 << $bit) != 0
        }
    };
}
