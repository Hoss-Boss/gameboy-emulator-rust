use std::usize;

pub struct CPU {
SP: u16,
PC: u16,
A: u8,
B: u8,
C: u8,
D: u8,
E: u8,
H: u8,
L: u8,
FLAGS: u8,
}

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    FLAGS,
    AF,
    BC,
    DE,
    HL,
    PC,
    SP
}

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    NOP = 0x00,
    LD_BC_d16 = 0x01,
    LD_AddressBC_A = 0x02,
    INC_BC = 0x03,
    INC_B = 0x04,
    DEC_B = 0x05,
    LD_B_d8 = 0x06,
    RLCA = 0x07,
    LD_Address_SP = 0x08,
    LD_A_d8 = 0x3E,
    LD_C_d8 = 0x0E,
    LD_D_d8 = 0x16,
    LD_H_d8 = 0x26,
    LD_L_d8 = 0x2E,
    LD_HL_d8 = 0x36,
    JumpToImmediateAddress = 0xC3,
}

pub fn extract_opcode_from_byte(byte: u8) -> Opcode {
        match byte {
            0x00 => Opcode::NOP,
            0x01 => Opcode::LD_BC_d16,
            0x02 => Opcode::LD_AddressBC_A,
            0x03 => Opcode::INC_BC,
            0x04 => Opcode::INC_B,
            0x05 => Opcode::DEC_B,
            0x06 => Opcode::LD_B_d8,
            0x07 => Opcode::RLCA,
            0x08 => Opcode::LD_Address_SP,
            0xC3 => Opcode::JumpToImmediateAddress,
            0x3E => Opcode::LD_A_d8,
            _ => return Opcode::NOP,
        }
    }

const LOW_BITS_MASK_FOR_16_BIT:u16 = 0x00FF;
const HIGH_BITS_MASK_FOR_8_BIT:u8 = 0xF0;

const Z_FLAG_MASK: u8 = 0b1000_0000;
const N_FLAG_MASK: u8 = 0b0100_0000;
const H_FLAG_MASK: u8 = 0b0010_0000;
const C_FLAG_MASK: u8 = 0b0001_0000;

impl CPU {
    pub fn create() -> CPU {
        return CPU{A: 0, SP: 0, PC: 0, B: 0, C: 0, D: 0, E: 0, H: 0, L: 0, FLAGS: 0};
    }

    pub fn get_AF(&self) -> u16 {
        //Move the A register to the left by 8 bits
        //Set the FLAGS register as the right 8 bits
        return ((self.A as u16) << 8) | (self.FLAGS as u16)
    }

    pub fn get_BC(&self) -> u16 {
        return ((self.B as u16) << 8) | (self.C as u16)
    }

    pub fn get_DE(&self) -> u16 {
        return ((self.D as u16) << 8) | (self.E as u16)
    }

    pub fn get_HL(&self) -> u16 {
        return ((self.H as u16) << 8) | (self.L as u16)
    }

    pub fn get_PC(&self) -> u16 {
        return self.PC;
    }

    pub fn get_SP(&self) -> u16 {
        return self.SP;
    }

    pub fn get_A(&self) -> u8 {
        return self.A;
    }

    pub fn get_B(&self) -> u8 {
        return self.B;
    }

    pub fn get_C(&self) -> u8 {
        return self.C;
    }

    pub fn get_D(&self) -> u8 {
        return self.D;
    }

    pub fn get_E(&self) -> u8 {
        return self.E;
    }

    pub fn get_H(&self) -> u8 {
        return self.H;
    }

    pub fn get_L(&self) -> u8 {
        return self.L;
    }

    pub fn get_FLAGS(&self) -> u8 {
        return self.FLAGS;
    }

    fn set_AF(&mut self, value:u16) {
        self.A = (value >> 8) as u8;
        let low_bits = (value & LOW_BITS_MASK_FOR_16_BIT) as u8;
        self.FLAGS = low_bits & HIGH_BITS_MASK_FOR_8_BIT;
    }

    fn set_BC(&mut self, value: u16) {
        self.B = (value >> 8) as u8;
        self.C = value as u8;
    }

    fn set_DE(&mut self, value: u16) {
        self.D = (value >> 8) as u8;
        self.E = value as u8;
    }

    fn set_HL(&mut self, value: u16) {
        self.H = (value >> 8) as u8;
        self.L = value as u8;
    }

    fn set_SP(&mut self, value: u16) {
        self.SP = value;
    }

    fn set_PC(&mut self, value: u16) {
        self.PC = value;
    }

    pub fn advance_PC(&mut self, value: u16) {
        self.PC = self.PC.wrapping_add(value);
    }

    pub fn increment_8_bit_register(&mut self, register: Register) {
        match register {
            Register::A => self.set_A(self.get_A().wrapping_add(1)),
            Register::B => self.set_B(self.get_B().wrapping_add(1)),
            Register::C => self.set_C(self.get_C().wrapping_add(1)),
            Register::D => self.set_D(self.get_D().wrapping_add(1)),
            Register::E => self.set_E(self.get_E().wrapping_add(1)),
            Register::H => self.set_H(self.get_H().wrapping_add(1)),
            Register::L => self.set_L(self.get_L().wrapping_add(1)),
            Register::FLAGS => self.set_FLAGS(self.get_FLAGS().wrapping_add(1)),
            _ => println!("Not an 8-bit register"),
        }
    }

    pub fn decrement_8_bit_register(&mut self, register: Register) {
        match register {
            Register::A => self.set_A(self.get_A().wrapping_sub(1)),
            Register::B => self.set_B(self.get_B().wrapping_sub(1)),
            Register::C => self.set_C(self.get_C().wrapping_sub(1)),
            Register::D => self.set_D(self.get_D().wrapping_sub(1)),
            Register::E => self.set_E(self.get_E().wrapping_sub(1)),
            Register::H => self.set_H(self.get_H().wrapping_sub(1)),
            Register::L => self.set_L(self.get_L().wrapping_sub(1)),
            Register::FLAGS => self.set_FLAGS(self.get_FLAGS().wrapping_sub(1)),
            _ => println!("Not an 8-bit register"),
        }
    }

    

    pub fn increment_16_bit_register(&mut self, register: Register) {
        match register {
            Register::AF => self.set_AF(self.get_AF().wrapping_add(1)),
            Register::BC => self.set_BC(self.get_BC().wrapping_add(1)),
            Register::DE => self.set_DE(self.get_DE().wrapping_add(1)),
            Register::HL => self.set_HL(self.get_HL().wrapping_add(1)),
            Register::SP => self.set_SP(self.get_SP().wrapping_add(1)),
            Register::PC => self.set_PC(self.get_PC().wrapping_add(1)),
            _ => println!("Not a 16-bit register"),
        }
    }

    pub fn decrement_16_bit_register(&mut self, register: Register) {
        match register {
            Register::AF => self.set_AF(self.get_AF().wrapping_sub(1)),
            Register::BC => self.set_BC(self.get_BC().wrapping_sub(1)),
            Register::DE => self.set_DE(self.get_DE().wrapping_sub(1)),
            Register::HL => self.set_HL(self.get_HL().wrapping_sub(1)),
            Register::SP => self.set_SP(self.get_SP().wrapping_sub(1)),
            Register::PC => self.set_PC(self.get_PC().wrapping_sub(1)),
            _ => println!("Not a 16-bit register"),
        }
    }

    fn set_C_flag(&mut self) {
        //Bit 4 of the flags register is the C flag
        //This requires bit shifting by 4 bits
        //00000001 to
        //00010000
        let current_flag_byte = self.get_FLAGS();
        let byte = 1 as u8;
        let new_flag_byte = current_flag_byte | ((byte << 4) & C_FLAG_MASK);
        self.set_FLAGS(new_flag_byte);
    }

    fn clear_C_flag(&mut self) {
        let current_flag_byte = self.get_FLAGS();
        let new_flag_byte = current_flag_byte | ((current_flag_byte << 4) & !C_FLAG_MASK);
        self.set_FLAGS(new_flag_byte);
    }

    fn set_A(&mut self, value: u8) {
        self.A = value;
    }

    fn set_B(&mut self, value: u8) {
        self.B = value;
    }

    fn set_C(&mut self, value: u8) {
        self.C = value;
    }

    fn set_D(&mut self, value: u8) {
        self.D = value;
    }

    fn set_E(&mut self, value: u8) {
        self.E = value;
    }

    fn set_H(&mut self, value: u8) {
        self.H = value;
    }

    fn set_L(&mut self, value: u8) {
        self.L = value;
    }

    fn set_FLAGS(&mut self, value: u8) {
        self.FLAGS = value;
    }



    pub fn set_8_bit_register(&mut self, register: Register, value: u8) {
         match register {
            Register::A => self.set_A(value),
            Register::B => self.set_B(value),
            Register::C => self.set_C(value),
            Register::D => self.set_D(value),
            Register::E => self.set_E(value),
            Register::H => self.set_H(value),
            Register::L => self.set_L(value),
            Register::FLAGS => self.set_FLAGS(value),
            _ => println!("Error: Not an 8 bit register.")
        }
    }

    pub fn set_16_bit_register(&mut self, register: Register, value: u16) {
        match register {
            Register::AF => self.set_AF(value),
            Register::BC => self.set_BC(value),
            Register::DE => self.set_DE(value),
            Register::HL => self.set_HL(value),
            Register::SP => self.set_SP(value),
            Register::PC => self.set_PC(value),
            _ => println!("Error: Not a 16 bit register.")
        }
    }

    fn execute_NOP(&mut self) {
    //Yes - there's a CPU instruction that does nothing. Really.
    }

    pub fn execute_LD_immediate_8_bit(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.set_A(value),
            Register::B => self.set_B(value),
            Register::C => self.set_C(value),
            Register::D => self.set_D(value),
            Register::E => self.set_E(value),
            Register::H => self.set_H(value),
            Register::L => self.set_L(value),
            _ => println!("Error: not an 8 bit register."),
        }
    }

    pub fn execute_LD_immediate_16_bit(&mut self, register: Register, value: u16) {
        match register {
            Register::AF => self.set_AF(value),
            Register::BC => self.set_BC(value),
            Register::DE => self.set_DE(value),
            Register::HL => self.set_HL(value),
            Register::PC => self.set_PC(value),
            Register::SP => self.set_SP(value),
            _ => println!("Error: not an 8 bit register."),
        }

    }

    pub fn execute_jump(&mut self, address: u16) {
        self.set_PC(address);
    }

    fn fetch(&mut self, memory: &Vec<u8>) -> u8{
        //Fetch a byte then advance the PC register
        let byte = memory[self.get_PC() as usize];
        self.advance_PC(1);
        return byte;
    }


    pub fn fetch_two_bytes(&mut self, memory: &[u8]) -> [u8;2] {
        let pc = self.get_PC() as usize;
        let bytes:[u8; 2]= [memory[pc], memory[pc + 1]];
        self.advance_PC(2);
        return bytes;

    }

    pub fn convert_two_u8_to_one_u16(&mut self, two_bytes: &[u8; 2]) -> u16{
        let low_byte = two_bytes[0] as u16;
        let high_byte = two_bytes[1] as u16;
        return (high_byte << 8) | low_byte;
    }


    pub fn execute_instruction(&mut self, memory: &mut [u8], opcode: Opcode) {
        let pc = self.get_PC() as usize;
        //PC increment happens on fetch. The current PC value should be the byte after the opcode.
        match opcode {
            Opcode::NOP => self.execute_NOP(), // 0x00
            Opcode::LD_BC_d16 =>  {//0x01
                let next_two_bytes = self.fetch_two_bytes(memory);
                let value = self.convert_two_u8_to_one_u16(&next_two_bytes);
                println!("Executing LD BC {}", value);
                self.set_BC(value);
            },
            Opcode::LD_AddressBC_A => {//0x02
                let address_in_BC = self.get_BC() as usize;
                memory[address_in_BC] = self.get_A();
            },
            Opcode::INC_BC => {//0x03
                self.increment_16_bit_register(Register::BC);
            },
            Opcode::INC_B => {//0x04
                self.increment_8_bit_register(Register::B);
            },
            Opcode::DEC_B => {//0x05
                self.decrement_8_bit_register(Register::B);
            },
            Opcode::LD_A_d8 => {//0x06
                let value = memory[pc];
                self.set_A(value);
            },
            Opcode::RLCA => {

            },
            Opcode::LD_B_d8 => {
                let value = memory[pc];
                self.set_B(value);
            },
            Opcode::LD_C_d8 => {
                let value = memory[pc];
                self.set_C(value);
            },
            Opcode::LD_D_d8 => {
                let value = memory[pc];
                self.set_D(value);

            },
            Opcode::LD_HL_d8 => {
                let value = memory[pc] as u16;
                self.set_HL(value);
            },
            Opcode::LD_L_d8 => {
                let value = memory[pc];
                self.set_L(value);
            },
            Opcode::JumpToImmediateAddress => {
                let low_byte = memory[pc];
                let high_byte = memory[pc + 1];
                let address = ((high_byte as u16) << 8) | (low_byte as u16); 
                self.execute_jump(address);
            },


            _ => println!("Not implemented yet"),
        }
    }
}