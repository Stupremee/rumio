pub trait RegisterReadWrite {
    type Addr: Int;

    fn set(val: Self::Addr);

    fn get() -> Self::Addr;
}

pub trait Int {
    fn zero() -> Self;
}

enum Mstatus {}

impl RegisterReadWrite for Mstatus {
    type Addr = u32;

    fn write(val: Self::Addr) {
        unsafe { asm!("csrw mstatus {}", in(reg) val) }
    }

    fn read() -> Self::Addr {
        let val;
        unsafe { asm!("csrr {} mstatus", out(reg) val) }
        val
    }
}

define_cpu_register! { rw Mstatus =>
    rw UIE: 0,
    rw SIE: 1,
    rw MIE: 3,
    r UPIE: 4,

    r MPP: 11..12 = PrivilegeMode [
        User = 0b00,
        Supervisor = 0b01,
        Machine = 0b11,
    ],

    rw FLAGS: 14..16 = bitflags Flags [
        YES = 0b001,
        NO = 0b010,
        MAYBE = 0b100,
    ],
}

define_mmio_register! { rw IER =>
    rw DATA_READY: 0,
    rw THR_EMPTY: 1,
    rw RECV_LINE: 2,
}

define_mmio_register! { rw FCR =>
    rw FIFO_ENABLE: 0,
}

define_mmio_register! { rw LCR =>
    rw WORD_LEN: 0..1 = Length [
        Five = 0b00,
        Six = 0b01,
        Seven = 0b10,
        Eight = 0b11,
    ],

    rw DLAB: 7,
}

define_mmio! {
    pub struct Registers {
        ier: IER,
        _pad: Padding<u8, 8>,
        fcr: FCR,
        lcr: LCR,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
