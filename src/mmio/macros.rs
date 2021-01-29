/// Define abstractions for a single register inside an MMIO block.
///
/// The macro looks almost the same as [`define_cpu_register`](crate::define_cpu_register),
/// and all the field types and properties of the CPU register version apply here too.
///
/// **Note** that the generated struct for this register doesn't has the same layout
/// as the given number type, and should always be constructed using the `new` method.
///
/// # Example
///
/// ```
/// rumio::define_mmio_register! {
///     Reg: u16 {
///         rw MODE: 0..1 = enum Mode [
///             A = 0b00,
///             B = 0b01,
///             C = 0b10,
///             D = 0b11,
///         ],
///
///         r FOO: 2,
///
///         rw BAR: 3,
///         rw BAZ: 4,
///
///         rw FLAGS: 5..8 = flags Flags [
///             A = 0b0001,
///             B = 0b0010,
///             C = 0b0100,
///             D = 0b1000,
///         ],
///     }
/// }
/// ```
///
///
/// To explore the whole generated api, take a look at the
/// [`example_generated`](crate::example_generated) module on docs.rs
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
            #[derive(Clone, Copy)]
            pub struct $name($crate::mmio::VolAddr<$num_ty>);
        )*

        $(#[$reg_attr])*
        #[derive(Clone, Copy)]
        pub struct $reg_name($crate::mmio::VolAddr<$num_ty>);

        #[allow(dead_code)]
        impl $reg_name {
            /// Create a new instance of this register at the given address.
            #[inline]
            pub const fn new(addr: $crate::mmio::VolAddr<$num_ty>) -> Self {
                Self(addr)
            }

            $crate::__generate_if_perm__! { @read
                /// Get the raw value from this MMIO register.
                pub fn get(self) -> $num_ty {
                    $crate::mmio::VolAddr::<$num_ty>::read(self.0)
                }
                => $($perm) *
            }

            $crate::__generate_if_perm__! { @write
                /// Write the raw value into this MMIO register.
                pub fn set(self, val: $num_ty) {
                    $crate::mmio::VolAddr::<$num_ty>::write(self.0, val);
                }
                => $($perm) *
            }

            $crate::__generate_if_perm__! { @read
                /// Check if one of the given fields is set.
                ///
                /// Returns `true` if the value specified by the field is not null.
                pub fn is_set<P: $crate::perm::Permission>(self, field: $crate::Field<$num_ty, P>) -> ::core::primitive::bool {
                    let val = $crate::mmio::VolAddr::<$num_ty>::read(self.0);
                    $crate::Field::<$num_ty, P>::read(field, val) != 0
                }
                => $($perm) *
            }

            $crate::__generate_if_perm__! { @read
                /// Read the given field from this register.
                pub fn read<P: $crate::perm::Permission>(self, field: $crate::Field<$num_ty, P>) -> $num_ty {
                    let val = $crate::mmio::VolAddr::<$num_ty>::read(self.0);
                    $crate::Field::<$num_ty, P>::read(field, val)
                }
                => $($perm) *
            }

            $crate::__generate_if_perm__! { @write
                /// Write the given values into this register and set all other bits to 0.
                pub fn write(self, val: $crate::Value<$num_ty>) {
                    let val = $crate::Value::<$num_ty>::modify(val, 0);
                    $crate::mmio::VolAddr::<$num_ty>::write(self.0, val);
                }
                => $($perm) *
            }

            $crate::__generate_if_perm__! { @read_write
                /// Modify this register to match the given value, but keep all other bits untouched.
                pub fn modify(self, val: $crate::Value<$num_ty>) {
                    let reg = $crate::mmio::VolAddr::<$num_ty>::read(self.0);
                    let reg = $crate::Value::<$num_ty>::modify(val, reg);
                    $crate::mmio::VolAddr::<$num_ty>::write(self.0, reg);
                }
                => $($perm) *
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
                let val = $crate::mmio::VolAddr::<$num_ty>::read(self.0);
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
                let val = $crate::mmio::VolAddr::<$num_ty>::read(self.0);
                $crate::mmio::VolAddr::<$num_ty>::write(self.0, $crate::set_bits(val, ($from, $to), bits));
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
                let val = $crate::mmio::VolAddr::<$num_ty>::read(self.0);
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
                let val = $crate::mmio::VolAddr::<$num_ty>::read(self.0);
                let val = $crate::set_bits(val, ($from, $to), bits);
                $crate::mmio::VolAddr::<$num_ty>::write(self.0, val);
            }
        }
    };

    // =====================================
    // Read and write a single bit
    // =====================================

    (@internal, $num_ty:ty, $perm:ident $name:ident: $bit:literal) => {
        impl $name {
            /// A `Field` that covers this single bit.
            pub const FIELD: $crate::Field<$num_ty, $crate::__perm_for_name__!($perm)> = $crate::Field::<$num_ty, _>::new(1 << $bit);
        }

        $crate::define_mmio_register!(@internal_bit, $num_ty, $perm $name: $bit);
    };

    (@internal_bit, $num_ty:ty, rw $name:ident: $bit:literal) => {
        $crate::define_mmio_register!(@internal_bit, $num_ty, r $name: $bit);
        $crate::define_mmio_register!(@internal_bit, $num_ty, w $name: $bit);
    };

    (@internal_bit, $num_ty:ty, r $name:ident: $bit:literal) => {
        impl $name {
            /// Check if this bit is set inside the MMIO.
            #[allow(unused)]
            pub fn get(&self) -> ::core::primitive::bool {
                let val = $crate::mmio::VolAddr::<$num_ty>::read(self.0);
                val & (1 << $bit) != 0
            }
        }
    };

    (@internal_bit, $num_ty:ty, w $name:ident: $bit:literal) => {
        impl $name {
            /// A `Value` that will set this bit to high when modifying a register.
            pub const SET: $crate::Value<$num_ty> = $crate::Value::<$num_ty>::new(1 << $bit, 1 << $bit);

            /// A `Value` that will set this bit to low when modifying a register.
            pub const CLEAR: $crate::Value<$num_ty> = $crate::Value::<$num_ty>::new(1 << $bit, 0);

            /// Set the value of this bit inside the MMIO.
            #[allow(unused)]
            pub fn set(&self, x: ::core::primitive::bool) {
                const MASK: $num_ty = 1 << $bit;
                let val = $crate::mmio::VolAddr::<$num_ty>::read(self.0);
                let val = match x {
                    true => val | MASK,
                    false => val & !MASK,
                };
                $crate::mmio::VolAddr::<$num_ty>::write(self.0, val);
            }
        }
    };
}

/// Creates a struct which represents the MMIO block and all their registers.
///
/// **Note** that the generated struct **does not** has the same layout as
/// you provided in this macro. So it must not be created like this:
///
/// ```ignore
/// let registers = unsafe { &*(0x4000_8000 as *const MmioDevice) };
/// ```
///
/// You must use the `new` method that is generated.
///
/// # Example
///
/// ```
/// rumio::define_mmio_register! {
///     Reg: u16 {
///         rw MODE: 0..1 = enum Mode [
///             A = 0b00,
///             B = 0b01,
///             C = 0b10,
///             D = 0b11,
///         ],
///
///         r FOO: 2,
///
///         rw BAR: 3,
///         rw BAZ: 4,
///
///         rw FLAGS: 5..8 = flags Flags [
///             A = 0b0001,
///             B = 0b0010,
///             C = 0b0100,
///             D = 0b1000,
///         ],
///     }
/// }
///
/// rumio::define_mmio_struct! {
///     pub struct Device {
///         0x00 => one: Reg,
///         0x08 => two: Reg,
///     }
/// }
/// ```
///
///
/// To explore the whole generated api, take a look at the
/// [`example_generated`](crate::example_generated) module on docs.rs
#[macro_export]
macro_rules! define_mmio_struct {
    ($(#[$attr:meta])*
     $pub:vis struct $name:ident {$(
         $(#[$field_attr:meta])*
         $field_offset:expr => $field_name:ident: $field_ty:ty
    ),*$(,)?}) => {
        $(#[$attr])*
        #[derive(Clone, Copy)]
        $pub struct $name($crate::mmio::VolAddr<u8>);

        impl $name {
            /// Create a new MMIO region at the given address.
            ///
            /// # Safety
            ///
            /// The safety arguments of `VolAddr` and
            /// it's `new` method must be guaranteed.
            pub const unsafe fn new(addr: ::core::primitive::usize) -> Self {
                Self($crate::mmio::VolAddr::<u8>::new(addr))
            }

            $($(#[$field_attr])*
            #[allow(unused)]
            pub fn $field_name(&self) -> $field_ty {
                <$field_ty>::new(unsafe {
                    $crate::mmio::VolAddr::cast(
                        $crate::mmio::VolAddr::offset(self.0, $field_offset)
                    )
                })
            })*
        }
    };
}
