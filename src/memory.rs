pub struct Memory {
    rom_bank: [u8; 0x8000], // 32 KiB ROM bank, no MBC support for now
}

impl Memory {
    // constructor
    pub fn new() -> Memory {
        return Memory {
            rom_bank: [0; 0x8000],
        };
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let mbc_byte = rom[0x0147];
        println!(
            "CARTRIDGE TYPE : {}",
            match mbc_byte {
                0x00 => "ROM ONLY",
                0x01 => "MBC1",
                0x02 => "MBC1+RAM",
                0x03 => "MBC1+RAM+BATTERY",
                0x05 => "MBC2",
                0x06 => "MBC2+BATTERY",
                0x08 => "ROM+RAM",
                0x09 => "ROM+RAM+BATTERY",
                0x0B => "MMM01",
                0x0C => "MMM01+RAM",
                0x0D => "MMM01+RAM+BATTERY",
                0x0F => "MBC3+TIMER+BATTERY",
                0x10 => "MBC3+TIMER+RAM+BATTERY",
                0x11 => "MBC3",
                0x12 => "MBC3+RAM",
                0x13 => "MBC3+RAM+BATTERY",
                0x19 => "MBC5",
                0x1A => "MBC5+RAM",
                0x1B => "MBC5+RAM+BATTERY",
                0x1C => "MBC5+RUMBLE",
                0x1D => "MBC5+RUMBLE+RAM",
                0x1E => "MBC5+RUMBLE+RAM+BATTERY",
                0xFC => "POCKET CAMERA",
                0xFD => "BANDAI TAMA5",
                0xFE => "HuC3",
                0xFF => "HuC1+RAM+BATTERY",
                _ => "UNKNOWN",
            }
        );

        if mbc_byte != 0x00 {
            panic!("ROMS using a MEMORY BANK CONTROLLER are not yet supported !");
        }

        if rom.len() > self.rom_bank.len() {
            panic!("ROM is too big ! {} < {}", rom.len(), self.rom_bank.len());
        }

        self.rom_bank[..rom.len()].copy_from_slice(&rom);
    }

    /*
    // accessors
    pub fn read_byte(&self, address: usize) -> u8 {
        return self.memory[address];
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn read_word(&self, address: usize) -> u16 {
        u16::from_le_bytes(self.memory[address..(address + 2)].try_into().unwrap())
    }

    pub fn write_word(&mut self, address: usize, value: u16) {
        let value_bytes = value.to_le_bytes();
        self.memory[address] = value_bytes[0];
        self.memory[address + 1] = value_bytes[1];
    } */
}
