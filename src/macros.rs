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
            #[allow(non_snake_case, dead_code)]
            pub mod $name {
                $crate::define_cpu_register!(@internal, $num_ty, $register, $perm $name: $from $(.. $to =
                    $($bitflags)? $kind_name [
                        $($kind_variant = $kind_variant_val),*
                    ]
                )?);
            }
        )*
    };

    (@internal, $num_ty:ty, $register:ident, rw $name:ident: $bit:literal) => {
        $crate::define_cpu_register!(@internal, $num_ty, $register, r $name: $bit);
        $crate::define_cpu_register!(@internal, $num_ty, $register, w $name: $bit);
    };

    (@internal, $num_ty:ty, $register:ident, r $name:ident: $bit:literal) => {
        /// Check if this bit is set inside the CPU register.
        pub fn get() -> bool {
            let val = <super::$register as $crate::cpu::RegisterRead<$num_ty>>::read();
            val & (1 << $bit) != 0
        }
    };

    (@internal, $num_ty:ty, $register:ident, w $name:ident: $bit:literal) => {
        /// Set the value of this inside the CPU register.
        pub fn set(x: bool) {
            const MASK: $num_ty = 0 << $bit;
            match x {
                true => <super::$register as $crate::cpu::RegisterWrite<$num_ty>>::set(MASK),
                false => <super::$register as $crate::cpu::RegisterWrite<$num_ty>>::clear(MASK),
            }
        }
    };
}
