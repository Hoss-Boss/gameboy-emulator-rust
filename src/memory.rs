pub const NINTENDO_LOGO_MEMORY: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
];

pub enum MemoryBank {
    Bank1,
    Bank2,
    Bank3,
}

pub struct Memory{
    cartridge_header: [u8; 80],
    bank_1: [u8; 0x4000],
    bank_2: [u8; 0x4000],
    bank_3: [u8; 0x4000],
    active_bank: MemoryBank,
}

impl Memory {

pub fn new() -> Memory{
    let bank_1 = [0u8; 0x4000];
    let bank_2 = [0u8; 0x4000];
    let bank_3 = [0u8; 0x4000];
    let cartridge_header= [0u8; 80];
    return Memory{active_bank: MemoryBank::Bank1, bank_1: bank_1, bank_2: bank_2, bank_3: bank_3, cartridge_header: cartridge_header};
}

pub fn set_active_memory_bank(&mut self, bank: MemoryBank) {
    match bank {
        MemoryBank::Bank1 => self.active_bank = MemoryBank::Bank1,
        MemoryBank::Bank2 => self.active_bank = MemoryBank::Bank2,
        MemoryBank::Bank3 => self.active_bank = MemoryBank::Bank3,
    }
} 

pub fn get_active_memory_bank(&self) -> &[u8] {
    match self.active_bank {
        MemoryBank::Bank1 => return &self.bank_1,
        MemoryBank::Bank2 => return &self.bank_2,
        MemoryBank::Bank3 => return &self.bank_3,
    }

}

pub fn set_memory_bank(memory: &mut Memory, bank: MemoryBank) {
    match bank {
        MemoryBank::Bank1 => memory.active_bank = MemoryBank::Bank1,
        MemoryBank::Bank2 => memory.active_bank = MemoryBank::Bank2,
        MemoryBank::Bank3 => memory.active_bank = MemoryBank::Bank3,
    }
}

}