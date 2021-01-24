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
    pub struct Reg;

    impl crate::cpu::RegisterRead<u32> for Reg {
        fn read() -> u32 {
            0
        }
    }

    impl crate::cpu::RegisterWrite<u32> for Reg {
        fn write(val: u32) {}

        fn set(mask: u32) {}

        fn clear(mask: u32) {}
    }

    #[test]
    fn it_works() {
        super::define_cpu_register! { Reg as u32 =>
            rw UIE: 0,
            rw SIE: 1,
            rw MIE: 3,
            rw UPIE: 4,
        }
    }
}
