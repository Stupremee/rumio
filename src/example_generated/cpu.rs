//! Example code for the [`define_cpu_register`] code.

use crate::{
    cpu::{RegisterRead, RegisterWrite},
    define_cpu_register,
};

struct CpuRegister;

impl RegisterRead<u64> for CpuRegister {
    fn read() -> u64 {
        unimplemented!()
    }
}

impl RegisterWrite<u64> for CpuRegister {
    fn write(_val: u64) {
        unimplemented!()
    }

    fn set(mask: u64) {
        crate::impl_cpu_set!(Self, mask);
    }

    fn clear(mask: u64) {
        crate::impl_cpu_clear!(Self, mask);
    }
}

define_cpu_register! { CpuRegister as u64 =>
    rw MODE: 0..1 = enum Mode [
        A = 0b00,
        B = 0b01,
        C = 0b10,
        D = 0b11,
    ],

    r FOO: 2,

    rw BAR: 3,
    rw BAZ: 4,

    rw FLAGS: 5..8 = flags Flags [
        A = 0b0001,
        B = 0b0010,
        C = 0b0100,
        D = 0b1000,
    ],
}
