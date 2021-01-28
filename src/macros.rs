#[doc(hidden)]
#[macro_export]
macro_rules! __generate_field_kinds__ {
    ($num_ty:ty, $from:literal .. $to:literal,
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

        impl ::core::convert::From<$kind_name> for $crate::Value<$num_ty> {
            fn from(x: $kind_name) -> $crate::Value<$num_ty> {
                let mask = $crate::set_bits(0, ($from, $to), !0);
                let bits = match x {
                    $($kind_name::$variant => $variant_val,)*
                };
                $crate::Value::<$num_ty>::new(mask, bits << $from)
            }
        }

        impl ::core::convert::From<$kind_name> for $crate::Field<$num_ty> {
            fn from(x: $kind_name) -> $crate::Field<$num_ty> {
                let mask = $crate::set_bits(0, ($from, $to), !0);
                $crate::Field::<$num_ty>::new(mask)
            }
        }

        impl ::core::ops::BitOr<$crate::Value<$num_ty>> for $kind_name {
            type Output = $crate::Value<$num_ty>;

            fn bitor(self, rhs: $crate::Value<$num_ty>) -> Self::Output {
                $crate::Value::<$num_ty>::from(self) | rhs
            }
        }

        impl ::core::ops::BitOr<$kind_name> for $crate::Value<$num_ty> {
            type Output = $crate::Value<$num_ty>;

            fn bitor(self, rhs: $kind_name) -> Self::Output {
                <$kind_name as ::core::ops::BitOr<$crate::Value<$num_ty>>>::bitor(rhs, self)
            }
        }

        impl ::core::ops::BitOr<$crate::Field<$num_ty>> for $kind_name {
            type Output = $crate::Field<$num_ty>;

            fn bitor(self, rhs: $crate::Field<$num_ty>) -> Self::Output {
                $crate::Field::<$num_ty>::from(self) | rhs
            }
        }

        impl ::core::ops::BitOr<$kind_name> for $crate::Field<$num_ty> {
            type Output = $crate::Field<$num_ty>;

            fn bitor(self, rhs: $kind_name) -> Self::Output {
                <$kind_name as ::core::ops::BitOr<$crate::Field<$num_ty>>>::bitor(rhs, self)
            }
        }
    };

    ($num_ty:ty, $from:literal .. $to:literal,
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

        impl ::core::convert::From<$kind_name> for $crate::Value<$num_ty> {
            fn from(x: $kind_name) -> $crate::Value<$num_ty> {
                let mask = $crate::set_bits(0, ($from, $to), !0);
                $crate::Value::<$num_ty>::new(mask, $kind_name::bits(&x) << $from)
            }
        }

        impl ::core::convert::From<$kind_name> for $crate::Field<$num_ty> {
            fn from(x: $kind_name) -> $crate::Field<$num_ty> {
                let mask = $crate::set_bits(0, ($from, $to), !0);
                $crate::Field::<$num_ty>::new(mask)
            }
        }

        impl ::core::ops::BitOr<$crate::Value<$num_ty>> for $kind_name {
            type Output = $crate::Value<$num_ty>;

            fn bitor(self, rhs: $crate::Value<$num_ty>) -> Self::Output {
                $crate::Value::<$num_ty>::from(self) | rhs
            }
        }

        impl ::core::ops::BitOr<$kind_name> for $crate::Value<$num_ty> {
            type Output = $crate::Value<$num_ty>;

            fn bitor(self, rhs: $kind_name) -> Self::Output {
                <$kind_name as ::core::ops::BitOr<$crate::Value<$num_ty>>>::bitor(rhs, self)
            }
        }

        impl ::core::ops::BitOr<$crate::Field<$num_ty>> for $kind_name {
            type Output = $crate::Field<$num_ty>;

            fn bitor(self, rhs: $crate::Field<$num_ty>) -> Self::Output {
                $crate::Field::<$num_ty>::from(self) | rhs
            }
        }

        impl ::core::ops::BitOr<$kind_name> for $crate::Field<$num_ty> {
            type Output = $crate::Field<$num_ty>;

            fn bitor(self, rhs: $kind_name) -> Self::Output {
                <$kind_name as ::core::ops::BitOr<$crate::Field<$num_ty>>>::bitor(rhs, self)
            }
        }
    };
}

/// Hidden macro that allows to generate a function
/// if at least one of the bitfields can be read/write.
#[doc(hidden)]
#[macro_export]
macro_rules! __generate_if_perm__ {
    (@read $code:item => $($perms:tt)*) => {
        $crate::__generate_if_perm__!(@internal_read $code => $($perms)*);
    };

    (@internal_read $code:item =>) => {};
    (@internal_read $code:item => r $($perms:tt)*) => { $code };
    (@internal_read $code:item => rw $($perms:tt)*) => { $code };
    (@internal_read $code:item => $_:tt $($perms:tt)*) => {
        $crate::__generate_if_perm__!(@internal_read $code => $($perms)*);
    };

    (@write $code:item => $($perms:tt)*) => {
        $crate::__generate_if_perm__!(@internal_write $code => $($perms)*);
    };

    (@internal_write $code:item =>) => {};
    (@internal_write $code:item => w $($perms:tt)*) => { $code };
    (@internal_write $code:item => rw $($perms:tt)*) => { $code };
    (@internal_write $code:item => $_:tt $($perms:tt)*) => {
        $crate::__generate_if_perm__!(@internal_write $code => $($perms)*);
    };

    (@read_write $code:item => $($perms:tt)*) => {
        $crate::__generate_if_perm__!(@internal_read_write $code => $($perms)*);
    };

    (@internal_read_write $code:item => rw $($perms:tt)*) => { $code };
    (@internal_read_write $code:item => r $($perms:tt)*) => {
        $crate::__generate_if_perm__!(@internal_read_write_r $code => $($perms)*);
    };
    (@internal_read_write $code:item => w $($perms:tt)*) => {
        $crate::__generate_if_perm__!(@internal_read_write_w $code => $($perms)*);
    };

    (@internal_read_write_r $code:item =>) => {};
    (@internal_read_write_r $code:item => w $($perms:tt)*) => { $code };
    (@internal_read_write_r $code:item => $_:tt $($perms:tt)*) => {
        $crate::__generate_if_perm__!(@internal_read_write_r $code => $($perms)*);
    };

    (@internal_read_write_w $code:item =>) => {};
    (@internal_read_write_w $code:item => r $($perms:tt)*) => { $code };
    (@internal_read_write_w $code:item => $_:tt $($perms:tt)*) => {
        $crate::__generate_if_perm__!(@internal_read_write_w $code => $($perms)*);
    };
}
