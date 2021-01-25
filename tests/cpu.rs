use rumio::cpu::{RegisterRead, RegisterWrite};
use std::sync::atomic::{AtomicU64, Ordering};

static REGISTER: AtomicU64 = AtomicU64::new(1);

struct CpuRegister;

impl RegisterRead<u64> for CpuRegister {
    fn read() -> u64 {
        REGISTER.load(Ordering::SeqCst)
    }
}

impl RegisterWrite<u64> for CpuRegister {
    fn write(val: u64) {
        REGISTER.store(val, Ordering::SeqCst);
    }

    fn set(mask: u64) {
        rumio::impl_cpu_set!(Self, mask);
    }

    fn clear(mask: u64) {
        rumio::impl_cpu_clear!(Self, mask);
    }
}

rumio::define_cpu_register! { CpuRegister as u64 =>
    r FOO: 0,

    rw BAR: 3,
    rw BAZ: 4,
}

#[test]
fn read_write_single_bit() {
    assert!(FOO::get());

    assert!(!BAR::get());
    assert!(!BAZ::get());

    BAR::set(true);
    assert!(BAR::get());
    assert!(!BAZ::get());

    BAZ::set(true);
    assert!(BAZ::get());
}
