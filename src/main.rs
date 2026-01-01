mod cpu;
mod memory;
use cpu::CPU;
use cpu::Register;
use cpu::extract_opcode_from_byte;
use std::fs;

use memory::Memory;
use memory::MemoryBank;

struct GameBoy {
    CPU: CPU,
    Memory: Vec<u8>,
}

fn main() {
    let mut cpu = CPU::create();
    cpu.execute_LD_immediate_8_bit(Register::A, 255);
    println!("{}", cpu.get_A());
    let mut memory = Memory::new();
    memory.set_active_memory_bank(MemoryBank::Bank3);
    Memory::set_memory_bank(&mut memory, MemoryBank::Bank2);
    //let rom_bytes = fs::read("SuperMarioLand.gb").expect("Failed to read SuperMarioLand.gb. Is it in the project root?");
    let mut rom_bytes: [u8; 3] = [0x01, 0x02, 0x01];
    println!("First instruction: 0x{:02X}", rom_bytes[0]);
    let opcode = extract_opcode_from_byte(rom_bytes[2]);
    cpu.execute_instruction(&mut rom_bytes, opcode);
    println!("Opcode: {:?}", opcode);
    println!("State of BC: {}", cpu.get_BC());
}