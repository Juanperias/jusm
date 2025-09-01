// note: apply & 0x0FFF to imm in addi

pub mod encode;
use object::SectionKind;

use crate::{elf::obj::Elf, parser::ast::AstNode};

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
        _ => Vec::new(),
    }
}

pub fn encode_sections(sections: Vec<AstNode>, elf: &mut Elf) {
    for section in sections {
        match section {
            AstNode::Section { name, content } => {
                let opts = match name.as_str() {
                    ".text" => ("text", SectionKind::Text),
                    ".data" => ("data", SectionKind::Data),
                    ".bss" => ("bss", SectionKind::UninitializedData),
                    other => (other, SectionKind::Unknown),
                };

                let section = elf.create_section(opts.0.to_string(), opts.1);
                let mut opcodes = Vec::new();

                for node in content {
                    opcodes.extend(encode(node));
                }

                elf.write_section(section.id, &opcodes, 4);
            }
            _ => {}
        }
    }
}
