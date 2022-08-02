pub enum Inst {
    Lui {
        rd: usize,
        imm: i32,
    },
    Auipc {
        rd: usize,
        imm: i32,
    },

    Jal {
        rd: usize,
        imm: i32,
    },
    Jalr {
        rd: usize,
        rs1: usize,
        imm: i32,
    },

    Beq {
        rs1: usize,
        rs2: usize,
        imm: i32,
    },
    Bne {
        rs1: usize,
        rs2: usize,
        imm: i32,
    },
    Blt {
        rs1: usize,
        rs2: usize,
        imm: i32,
    },
    Bge {
        rs1: usize,
        rs2: usize,
        imm: i32,
    },
    Bltu {
        rs1: usize,
        rs2: usize,
        imm: i32,
    },
    Bgeu {
        rs1: usize,
        rs2: usize,
        imm: i32,
    },

    Lb {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    Lh {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    Lw {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    Lbu {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    Lhu {
        rd: usize,
        rs1: usize,
        imm: i32,
    },

    Addi {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    Slti {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    Sltiu {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    Xori {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    Ori {
        rd: usize,
        rs1: usize,
        imm: i32,
    },
    Andi {
        rd: usize,
        rs1: usize,
        imm: i32,
    },

    Slli {
        rd: usize,
        rs1: usize,
        shamt: u32,
    },
    Srli {
        rd: usize,
        rs1: usize,
        shamt: u32,
    },
    Srai {
        rd: usize,
        rs1: usize,
        shamt: u32,
    },

    Add {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Sub {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Sll {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Slt {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Sltu {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Xor {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Or {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    And {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },

    Srl {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },
    Sra {
        rd: usize,
        rs1: usize,
        rs2: usize,
    },

    Fence {
        rd: usize,
        rs1: usize,
        rs2: usize,
        succ: u32,
        pred: u32,
        fm: u32,
    },

    Ecall,
    Ebreak,
}

enum ImmType {
    R,
    I,
    S,
    B,
    U,
    J,
}

impl ImmType {
    pub fn decode(&self, inst: u32) -> Result<Inst, ()> {
        use Inst::*;

        let opcode = inst & 0b1111111;

        match self {
            ImmType::R => todo!(),
            ImmType::I => todo!(),
            ImmType::S => todo!(),
            ImmType::B => todo!(),
            ImmType::U => {
                let imm = (inst & 0xfffff000) as i32;
                let rd = ((inst >> 7) & 0b11111) as usize;

                match opcode {
                    0b0110111 => Ok(Lui { rd, imm }),
                    _ => Err(()),
                }
            }
            ImmType::J => todo!(),
        }
    }
}

const INST_TABLE: [Option<ImmType>; 128] = [
    /* 0b0000000 */ None,
    /* 0b0000001 */ None,
    /* 0b0000010 */ None,
    /* 0b0000011 */ Some(ImmType::I),
    /* 0b0000100 */ None,
    /* 0b0000101 */ None,
    /* 0b0000110 */ None,
    /* 0b0000111 */ None,
    /* 0b0001000 */ None,
    /* 0b0001001 */ None,
    /* 0b0001010 */ None,
    /* 0b0001011 */ None,
    /* 0b0001100 */ None,
    /* 0b0001101 */ None,
    /* 0b0001110 */ None,
    /* 0b0001111 */ Some(ImmType::I),
    /* 0b0010000 */ None,
    /* 0b0010001 */ None,
    /* 0b0010010 */ None,
    /* 0b0010011 */ Some(ImmType::I),
    /* 0b0010100 */ None,
    /* 0b0010101 */ None,
    /* 0b0010110 */ None,
    /* 0b0010111 */ Some(ImmType::U),
    /* 0b0011000 */ None,
    /* 0b0011001 */ None,
    /* 0b0011010 */ None,
    /* 0b0011011 */ None,
    /* 0b0011100 */ None,
    /* 0b0011101 */ None,
    /* 0b0011110 */ None,
    /* 0b0011111 */ None,
    /* 0b0100000 */ None,
    /* 0b0100001 */ None,
    /* 0b0100010 */ None,
    /* 0b0100011 */ Some(ImmType::S),
    /* 0b0100100 */ None,
    /* 0b0100101 */ None,
    /* 0b0100110 */ None,
    /* 0b0100111 */ None,
    /* 0b0101000 */ None,
    /* 0b0101001 */ None,
    /* 0b0101010 */ None,
    /* 0b0101011 */ None,
    /* 0b0101100 */ None,
    /* 0b0101101 */ None,
    /* 0b0101110 */ None,
    /* 0b0101111 */ None,
    /* 0b0110000 */ None,
    /* 0b0110001 */ None,
    /* 0b0110010 */ None,
    /* 0b0110011 */ Some(ImmType::R),
    /* 0b0110100 */ None,
    /* 0b0110101 */ None,
    /* 0b0110110 */ None,
    /* 0b0110111 */ Some(ImmType::U),
    /* 0b0111000 */ None,
    /* 0b0111001 */ None,
    /* 0b0111010 */ None,
    /* 0b0111011 */ None,
    /* 0b0111100 */ None,
    /* 0b0111101 */ None,
    /* 0b0111110 */ None,
    /* 0b0111111 */ None,
    /* 0b1000000 */ None,
    /* 0b1000001 */ None,
    /* 0b1000010 */ None,
    /* 0b1000011 */ None,
    /* 0b1000100 */ None,
    /* 0b1000101 */ None,
    /* 0b1000110 */ None,
    /* 0b1000111 */ None,
    /* 0b1001000 */ None,
    /* 0b1001001 */ None,
    /* 0b1001010 */ None,
    /* 0b1001011 */ None,
    /* 0b1001100 */ None,
    /* 0b1001101 */ None,
    /* 0b1001110 */ None,
    /* 0b1001111 */ None,
    /* 0b1010000 */ None,
    /* 0b1010001 */ None,
    /* 0b1010010 */ None,
    /* 0b1010011 */ None,
    /* 0b1010100 */ None,
    /* 0b1010101 */ None,
    /* 0b1010110 */ None,
    /* 0b1010111 */ None,
    /* 0b1011000 */ None,
    /* 0b1011001 */ None,
    /* 0b1011010 */ None,
    /* 0b1011011 */ None,
    /* 0b1011100 */ None,
    /* 0b1011101 */ None,
    /* 0b1011110 */ None,
    /* 0b1011111 */ None,
    /* 0b1100000 */ None,
    /* 0b1100001 */ None,
    /* 0b1100010 */ None,
    /* 0b1100011 */ Some(ImmType::B),
    /* 0b1100100 */ None,
    /* 0b1100101 */ None,
    /* 0b1100110 */ None,
    /* 0b1100111 */ Some(ImmType::I),
    /* 0b1101000 */ None,
    /* 0b1101001 */ None,
    /* 0b1101010 */ None,
    /* 0b1101011 */ None,
    /* 0b1101100 */ None,
    /* 0b1101101 */ None,
    /* 0b1101110 */ None,
    /* 0b1101111 */ Some(ImmType::J),
    /* 0b1110000 */ None,
    /* 0b1110001 */ None,
    /* 0b1110010 */ None,
    /* 0b1110011 */ Some(ImmType::I),
    /* 0b1110100 */ None,
    /* 0b1110101 */ None,
    /* 0b1110110 */ None,
    /* 0b1110111 */ None,
    /* 0b1111000 */ None,
    /* 0b1111001 */ None,
    /* 0b1111010 */ None,
    /* 0b1111011 */ None,
    /* 0b1111100 */ None,
    /* 0b1111101 */ None,
    /* 0b1111110 */ None,
    /* 0b1111111 */ None,
];
