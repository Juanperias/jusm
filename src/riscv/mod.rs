// note: apply & 0x0FFF to imm in addi

pub mod encode;
use std::path::Path;

use object::{SectionKind, SymbolKind};

use crate::{
    elf::obj::Elf,
    parser::ast::AstNode,
    riscv::encode::{RegArgs, register},
};

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
        AstNode::Sub { rd, rs1, rs2 } => register(RegArgs {
            rs1,
            rs2,
            rd,
            funct7: 0x20,
            funct3: 0x0,
            opcode: 0b0110011,
        }),
        AstNode::Sll { rd, rs1, rs2 } => register(RegArgs {
            rs1,
            rs2,
            rd,
            funct7: 0x0,
            funct3: 0x1,
            opcode: 0b0110011,
        }),
        AstNode::Xor { rd, rs1, rs2 } => register(RegArgs { 
            rd,
            rs1,
            rs2,
            funct7: 0x0,
            funct3: 0x4,
            opcode: 0b0110011,
        }),
        AstNode::Add { rd, rs1, rs2 } => register(RegArgs {
            rs1,
            rs2,
            rd,
            funct3: 0,
            funct7: 0,
            opcode: 0b0110011,
        }),
        _ => Vec::new(),
    }
}

fn section_opts(name: &str) -> (&str, SectionKind) {
    match name {
        ".text" => ("text", SectionKind::Text),
        ".data" => ("data", SectionKind::Data),
        ".bss" => ("bss", SectionKind::UninitializedData),
        other => (other, SectionKind::Unknown),
    }
}

fn encode_label(a: &mut Elf, section_name: &str, name: String, content: Vec<AstNode>) {
    let mut symbol_content = Vec::new();
    for node in content {
        symbol_content.extend(encode(node));
    }
    a.create_symbol(
        section_name.to_string(),
        name,
        SymbolKind::Text,
        &symbol_content,
        4,
    );
}

pub fn encode_sections<'a>(sections: Vec<AstNode>) -> Elf<'a> {
    let mut elf = Elf::new();

    for section in sections {
        if let AstNode::Section { name, content } = section {
            let (sec_name, sec_kind) = section_opts(&name);

            let (id, section_name) = {
                let s = elf.create_section(sec_name.to_string(), sec_kind);

                (s.id, s.name.clone())
            };
            let mut opcodes = Vec::new();

            for node in content {
                match node {
                    AstNode::Label { name, content } => {
                        println!("{:?}", name);
                        encode_label(&mut elf, &section_name, name, content);
                    }
                    n => opcodes.extend(encode(n)),
                }
            }

            elf.write_section(id, &opcodes, 4);
        }
    }

    elf
}
