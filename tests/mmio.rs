use rumio::mmio::Lit;
use std::{mem::ManuallyDrop, ptr};

struct MmioRegion {
    ptr: *mut u8,
    len: usize,
    cap: usize,
}

impl MmioRegion {
    fn new(size: usize) -> (MmioRegion, usize) {
        let mut region = ManuallyDrop::new(vec![0u8; size]);
        let (ptr, len, cap) = (region.as_mut_ptr(), region.len(), region.capacity());

        (Self { ptr, len, cap }, ptr as usize)
    }
}

impl Drop for MmioRegion {
    fn drop(&mut self) {
        let _region = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.cap) };
    }
}

rumio::define_mmio_register! {
    Reg: u16 {
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
}

rumio::define_mmio_struct! {
    pub struct Device {
        0x00 => one: Reg,
        0x08 => two: Reg,
        0x0A => lit1: Lit<u32>,
        0x0E => lit2: Lit<u8>,
        0x0F => lit3: [Lit<u8>; 32],
    }
}

#[test]
fn read_write_single_bit() {
    let (_guard, addr) = MmioRegion::new(16);
    let mmio = unsafe { Device::new(addr) };

    assert!(!mmio.one().FOO().get());
    assert!(!mmio.one().BAZ().get());
    assert!(!mmio.one().BAR().get());

    unsafe { ptr::write_volatile(addr as *mut u8, 0b00010100) };

    assert!(mmio.one().FOO().get());
    assert!(mmio.one().BAZ().get());
    assert!(!mmio.one().BAR().get());

    mmio.two().BAR().set(true);
    assert!(mmio.two().BAR().get());
}

#[test]
fn read_write_enum() {
    let (_guard, addr) = MmioRegion::new(16);
    let mmio = unsafe { Device::new(addr) };
    unsafe { ptr::write_volatile(addr as *mut u8, 0b11) };

    assert_eq!(mmio.one().MODE().get(), Some(Mode::D));

    mmio.one().MODE().set(Mode::A);
    assert_eq!(mmio.one().MODE().get(), Some(Mode::A));
}

#[test]
fn read_write_flags() {
    let (_guard, addr) = MmioRegion::new(16);
    let mmio = unsafe { Device::new(addr) };
    unsafe { ptr::write_volatile(addr as *mut u16, 0b111000000) };

    assert_eq!(mmio.one().FLAGS().get(), Flags::B | Flags::C | Flags::D);

    mmio.one().FLAGS().set(Flags::B | Flags::C);
    assert_eq!(mmio.one().FLAGS().get(), Flags::B | Flags::C);

    mmio.two().FLAGS().set(Flags::A | Flags::C);
    assert_eq!(mmio.two().FLAGS().get(), Flags::A | Flags::C);
}

#[test]
fn read_write_lit() {
    let (_guard, addr) = MmioRegion::new(16);
    let mmio = unsafe { Device::new(addr) };

    assert_eq!(mmio.lit1().read(), 0);
    assert_eq!(mmio.lit2().read(), 0);

    mmio.lit1().write(0xF00D_BABE);
    mmio.lit2().write(0xAB);

    assert_eq!(mmio.lit1().read(), 0xF00D_BABE);
    assert_eq!(mmio.lit2().read(), 0xAB);
}

#[test]
fn read_write_array() {
    let (_guard, addr) = MmioRegion::new(48);
    let mmio = unsafe { Device::new(addr) };

    for idx in 0..32 {
        mmio.lit3(idx).write(idx as u8);
    }

    for idx in 0..32 {
        assert_eq!(mmio.lit3(idx).read(), idx as u8);
    }
}
