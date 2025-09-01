// note: apply & 0x0FFF to imm in addi

pub mod encode;
use crate::parser::ast::AstNode;

use self::encode::{ImmArgs, immediate};

pub fn encode(node: AstNode) -> Vec<u8> {
    match node {
        AstNode::Ecall => immediate(ImmArgs {
            imm: 0x0,
            rs1: 0x0,
            rd: 0x0,
            opcode: 0b1110011,
        }),
        AstNode::Addi { rd, rs1, imm } => immediate(ImmArgs {
            imm: imm & 0x0fff,
            rs1,
            rd,
            opcode: 0b0010011,
        }),
        AstNode::Section { .. } => Vec::new(),
    }
}

pub fn encode_nodes(nodes: Vec<AstNode>) -> Vec<u8> {
    let mut output = Vec::new();

    for node in nodes {
        output.extend(encode(node));
    }

    output
}
