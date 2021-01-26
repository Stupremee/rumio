rumio::define_mmio_register! {
    Reg1: u16 {
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
        0x00 => one: Reg1,
    }
}

#[test]
fn read_write_single_bit() {}
