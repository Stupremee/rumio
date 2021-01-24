pub mod cpu;

mod macros;

macro_rules! impl_int_primitives {
    ($($num:ty),*) => {
        $(impl $crate::Int for $num {})*
    };
}

/// Represents any type that can be used as
/// the underlying value for a register.
pub trait Int: Clone + Copy + sealed::Sealed {}

impl_int_primitives!(u8, u16, u32, u64);

mod sealed {
    macro_rules! impl_sealed {
        ($($num:ty),*) => {
            $(impl $crate::sealed::Sealed for $num {})*
        };
    }

    pub trait Sealed {}

    impl_sealed!(u8, u16, u32, u64);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        pub struct Reg;

        super::define_cpu_register! { Reg as u32 =>
            r UIE: 0,
            r SIE: 1,
            r MIE: 3,
            r UPIE: 4,
        }
    }
}
