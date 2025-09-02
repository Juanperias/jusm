// note: apply & 0x0FFF to imm in addi

pub mod encode;
use std::path::Path;

use object::{SectionKind, SymbolKind};

use crate::{elf::obj::Elf, parser::ast::AstNode, riscv::encode::{register, RegArgs}};

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

pub fn encode_sections<'a>(sections: Vec<AstNode>) -> Elf<'a> {
    let mut elf = Elf::new();

    for section in sections {
        match section {
            AstNode::Section { name, content } => {
                let opts = match name.as_str() {
                    ".text" => ("text", SectionKind::Text),
                    ".data" => ("data", SectionKind::Data),
                    ".bss" => ("bss", SectionKind::UninitializedData),
                    other => (other, SectionKind::Unknown),
                };

                let mut opcodes = Vec::new();

                let id = {
                    let a = &mut elf;
                    let (id, section_name) = {
                        let s = a.create_section(opts.0.to_string(), opts.1);

                        (s.id, s.name.clone())
                    };

                    for node in content {
                        match node {
                            AstNode::Label { name, content } => {
                                let mut symbol_content = Vec::new();

                                for node in content {
                                    symbol_content.extend(encode(node));
                                }

                                a.create_symbol(
                                    section_name.clone(),
                                    name,
                                    SymbolKind::Text,
                                    &symbol_content,
                                    4,
                                );
                            }
                            n => {
                                opcodes.extend(encode(n));
                            }
                        }
                    }

                    id
                };

                elf.write_section(id, &opcodes, 4);
            }
            _ => {}
        }
    }

    elf
}
