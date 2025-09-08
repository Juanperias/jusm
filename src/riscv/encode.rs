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

// 00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000

pub fn store(arg: StoreArgs) -> Vec<u8> {
    let ins = ((arg.imm & 0b00000000_00000000_00000000_00000000_00000000_00000000_00001111_11100000) as u32) << 25
        | arg.rs2 << 20
        | arg.rs1 << 15
        | arg.funct3 << 12
        | ((arg.imm & 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00011111) as u32) << 7
        | arg.opcode;

    ins.to_le_bytes().to_vec()   
}

pub fn immediate(arg: ImmArgs) -> Vec<u8> {
    let ins = ((arg.imm as u32) << 20)
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
