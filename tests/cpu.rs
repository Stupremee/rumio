use rumio::cpu::{RegisterRead, RegisterWrite};
use std::sync::atomic::{AtomicU64, Ordering};

const DEFAULT_REG_VALUE: u64 = 0b101;

thread_local! {
    static REGISTER: AtomicU64 = AtomicU64::new(DEFAULT_REG_VALUE);
}

fn reset_register() {
    REGISTER.with(|reg| reg.store(DEFAULT_REG_VALUE, Ordering::SeqCst))
}

struct CpuRegister;

impl RegisterRead<u64> for CpuRegister {
    fn read() -> u64 {
        REGISTER.with(|reg| reg.load(Ordering::SeqCst))
    }
}

impl RegisterWrite<u64> for CpuRegister {
    fn write(val: u64) {
        REGISTER.with(|reg| reg.store(val, Ordering::SeqCst))
    }

    fn set(mask: u64) {
        rumio::impl_cpu_set!(Self, mask);
    }

    fn clear(mask: u64) {
        rumio::impl_cpu_clear!(Self, mask);
    }
}

rumio::define_cpu_register! { CpuRegister as u64 =>
    rw MODE: 0..1 = enum Mode [
        A = 0b00,
        B = 0b01,
        C = 0b10,
        D = 0b11,
    ],

    r FOO: 2,

    rw BAR: 3,
    rw BAZ: 4,
}

#[test]
fn read_write_single_bit() {
    reset_register();

    assert!(FOO::get());

    assert!(!BAR::get());
    assert!(!BAZ::get());

    BAR::set(true);
    assert!(BAR::get());
    assert!(!BAZ::get());

    BAZ::set(true);
    assert!(BAZ::get());
}

#[test]
fn read_write_enum() {
    reset_register();

    assert_eq!(MODE::get(), Some(Mode::B));

    MODE::set(Mode::C);
    assert_eq!(MODE::get(), Some(Mode::C));
}
