use crate::riscv::PC;

pub struct ImmArgs {
    pub imm: u64,
    pub rs1: u32,
    pub rd: u32,
    pub funct3: u32,
    pub opcode: u32,
}

pub struct RegArgs {
    pub rs1: u32,
    pub rs2: u32,
    pub rd: u32,
    pub funct7: u32,
    pub funct3: u32,
    pub opcode: u32,
}

pub struct StoreArgs {
    pub imm: u64,
    pub rs2: u32,
    pub rs1: u32,
    pub opcode: u32,
    pub funct3: u32,
}

pub struct UpperArgs {
    pub imm: u64,
    pub rd: u32,
    pub opcode: u32,
}

// 00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000

pub fn upper(arg: UpperArgs) -> Vec<u8> {
    let ins = ((arg.imm & 0xFFFFF) as u32) << 12 | arg.rd << 7 | arg.opcode;

    ins.to_le_bytes().to_vec()
}

pub fn store(arg: StoreArgs) -> Vec<u8> {
    let ins = ((arg.imm & 0x0000000000000FE0) as u32) << 25
        | arg.rs2 << 20
        | arg.rs1 << 15
        | arg.funct3 << 12
        | ((arg.imm & 0x000000000000001F) as u32) << 7
        | arg.opcode;

    PC.fetch_add(4, std::sync::atomic::Ordering::SeqCst);

    ins.to_le_bytes().to_vec()
}

pub fn immediate(arg: ImmArgs) -> Vec<u8> {
    let ins = (((arg.imm & 0x0fff) as u32) << 20)
        | arg.rs1 << 15
        | arg.funct3 << 12
        | (arg.rd as u32) << 7
        | arg.opcode;

    ins.to_le_bytes().to_vec()
}

pub fn register(arg: RegArgs) -> Vec<u8> {
    let ins = arg.funct7 << 25
        | arg.rs2 << 20
        | arg.rs1 << 15
        | arg.funct3 << 12
        | arg.rd << 7
        | arg.opcode;

    ins.to_le_bytes().to_vec()
}
