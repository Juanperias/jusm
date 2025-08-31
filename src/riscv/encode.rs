pub struct ImmArgs {
    pub imm: u64,
    pub rs1: u32,
    pub rd: u32,
    pub opcode: u32,

}

pub fn immediate(arg: ImmArgs) -> Vec<u8> {
    let ins = ((arg.imm as u32) << 20)
        | ((arg.rs1) as u32) << 15;
    //    | ;

}

pub fn register() {}
