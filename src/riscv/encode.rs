pub struct ImmArgs {
    pub imm: u64,
    pub rs1: u32,
    pub rd: u32,
    pub opcode: u32,
}

pub fn immediate(arg: ImmArgs) -> Vec<u8> {
    let ins = ((arg.imm as u32) << 20)
        | (arg.rs1 as u32) << 15
        | 0x0_u32 << 12
        | (arg.rd as u32) << 7
        | arg.opcode;

    ins.to_le_bytes().to_vec()
}

pub fn register() {}
