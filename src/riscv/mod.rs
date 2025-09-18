// note: apply & 0x0FFF to imm in addi

pub mod encode;
use std::{
    collections::HashMap,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

use object::{
    SectionKind, SymbolKind,
    elf::{R_RISCV_HI20, R_RISCV_LO12_I},
    write::{SectionId, coff::Symbol},
};

use crate::{
    elf::obj::Elf,
    parser::ast::{AstNode, SymbolInfo, Visibility},
    riscv::encode::{RegArgs, StoreArgs, UpperArgs, register, store, upper},
};

use self::encode::{ImmArgs, immediate};

static PC: AtomicU64 = AtomicU64::new(0);

pub fn encode(node: AstNode, elf: &mut Elf, section_id: SectionId) -> Vec<u8> {
    let base = PC.load(Ordering::SeqCst);

    let result = match node {
        AstNode::Ecall => immediate(ImmArgs {
            imm: 0x0,
            rs1: 0x0,
            rd: 0x0,
            funct3: 0x0,
            opcode: 0b1110011,
        }),

        AstNode::Addi { rd, rs1, imm } => immediate(ImmArgs {
            imm: imm,
            rs1,
            rd,
            funct3: 0x0,
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

        AstNode::Srl { rd, rs1, rs2 } => register(RegArgs {
            rs1,
            rs2,
            rd,
            funct7: 0x0,
            funct3: 0x5,
            opcode: 0b0110011,
        }),

        AstNode::Sra { rd, rs1, rs2 } => register(RegArgs {
            rs1,
            rs2,
            rd,
            funct7: 0x20,
            funct3: 0x5,
            opcode: 0b0110011,
        }),

        AstNode::Slt { rd, rs1, rs2 } => register(RegArgs {
            rs1,
            rs2,
            rd,
            funct7: 0x0,
            funct3: 0x2,
            opcode: 0b0110011,
        }),

        AstNode::Sltu { rd, rs1, rs2 } => register(RegArgs {
            rs1,
            rs2,
            rd,
            funct7: 0x0,
            funct3: 0x3,
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

        AstNode::Sb { rs1, rs2, imm } => store(StoreArgs {
            rs1,
            rs2,
            funct3: 0,
            imm,
            opcode: 0b0100011,
        }),

        AstNode::Lui { rd, imm } => upper(UpperArgs {
            rd,
            imm,
            opcode: 0b0110111,
        }),

        AstNode::Auipc { rd, imm } => upper(UpperArgs {
            rd,
            imm,
            opcode: 0b0010111,
        }),

        AstNode::La { rd, ref symbol } => {
            let (sym_id, sym_val) = elf.get_symbol_id(symbol.to_owned());
            let mut ops = Vec::new();

            println!("{:x}", base);
            ops.extend(upper(UpperArgs {
                imm: sym_val,
                rd,
                opcode: 0b0110111,
            }));
            elf.reallocate(section_id, sym_id, base, 0, R_RISCV_HI20);

            PC.fetch_add(4, Ordering::SeqCst);

            let base = PC.load(Ordering::SeqCst);

            println!("{:x}", base);

            ops.extend(immediate(ImmArgs {
                imm: sym_val,
                rs1: rd,
                rd,
                funct3: 0x0,
                opcode: 0b0010011,
            }));

            elf.reallocate(section_id, sym_id, base, 0, R_RISCV_LO12_I);

            ops
        }

        AstNode::Assci { ref seq } => seq.to_vec(),

        _ => Vec::new(),
    };

    if !matches!(node, AstNode::Assci { .. }) {
        PC.fetch_add(4, Ordering::SeqCst);
    }

    result
}

fn section_opts(name: &str) -> (&str, SectionKind, SymbolKind) {
    match name {
        ".text" => ("text", SectionKind::Text, SymbolKind::Text),
        ".data" => ("data", SectionKind::Data, SymbolKind::Data),
        ".bss" => ("bss", SectionKind::UninitializedData, SymbolKind::Data),
        other => (other, SectionKind::Unknown, SymbolKind::Section),
    }
}

fn encode_label(
    a: &mut Elf,
    section_name: &str,
    name: String,
    content: Vec<AstNode>,
    kind: SymbolKind,
    symbol_info: &SymbolInfo,
) {
    let mut symbol_content = Vec::new();
    let id = a.search_section(section_name.to_string()).id;
    for node in content {
        symbol_content.extend(encode(node, a, id));
    }
    a.create_symbol(
        section_name.to_string(),
        name,
        kind,
        &symbol_content,
        4,
        symbol_info,
    );
}

pub fn encode_sections<'a>(
    sections: Vec<AstNode>,
    visibility_map: HashMap<String, SymbolInfo>,
) -> Elf<'a> {
    let mut elf = Elf::new();

    for section in sections.clone() {
        if let AstNode::Section { name, content } = section {
            let (sec_name, sec_kind, sym_kind) = section_opts(&name);

            let (id, section_name) = {
                let s = elf.create_section(sec_name.to_string(), sec_kind);

                (s.id, s.name.clone())
            };
            let mut opcodes = Vec::new();

            for node in content {
                match node {
                    AstNode::Label { name, content } => {
                        let tmp = SymbolInfo {
                            ..Default::default()
                        };
                        let visiblity = visibility_map.get(&name).unwrap_or(&tmp);

                        for c in &content {
                            // TODO: error if symbol not exists
                            match c {
                                AstNode::La { rd, symbol } => {
                                    if !elf.symbols.contains_key(symbol) {
                                        if !search_label_encode(
                                            symbol,
                                            &sections,
                                            &mut elf,
                                            &section_name,
                                            &visibility_map,
                                        ) {
                                            panic!("symbol not exist");
                                        }
                                    }

                                    // exists
                                }
                                _ => {}
                            }
                        }

                        encode_label(&mut elf, &section_name, name, content, sym_kind, visiblity);
                    }
                    n => opcodes.extend(encode(n, &mut elf, id)),
                }
            }

            elf.write_section(id, &opcodes, 4);
        }
    }

    elf
}

pub fn search_label_encode(
    label: &String,
    start_content: &Vec<AstNode>,
    elf: &mut Elf,
    section_name: &str,
    visibility_map: &HashMap<String, SymbolInfo>,
) -> bool {
    for c in start_content {
        match c {
            AstNode::Section { name, content } => {
                println!("{name}");
                if search_label_encode(label, content, elf, name, visibility_map) {
                    return true;
                }
            }
            AstNode::Label { name, content } => {
                if name == label {
                    let (sec_name, sec_kind, sym_kind) = section_opts(section_name);
                    if !elf.sections.contains_key(sec_name) {
                        elf.create_section(sec_name.to_string(), sec_kind);
                    }
                    let tmp = SymbolInfo {
                        ..Default::default()
                    };
                    let v = visibility_map.get(name).unwrap_or(&tmp);

                    encode_label(elf, sec_name, label.clone(), content.clone(), sym_kind, v);
                    return true;
                }
            }
            //AstNode::La { rd, symbol } => panic!("Double la is not allowed... for now"),
            _ => {}
        }
    }

    false
}
