#[doc(hidden)]
#[macro_export]
macro_rules! __generate_field_kinds__ {
    ($num_ty:ty,
        $(#[$attr:meta])*
        enum $kind_name:ident [$(
            $(#[$variant_attr:meta])*
            $variant:ident = $variant_val:expr
        ),*]
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[allow(dead_code)]
        pub enum $kind_name {
            $( $(#[$variant_attr])* $variant ),*
        }
    };

    ($num_ty:ty,
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
