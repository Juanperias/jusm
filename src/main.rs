// fix this code

use std::path::Path;

use logos::Logos;

use crate::{
    elf::obj::Elf,
    parser::{
        ast::{AstNode, nodes_from_tokens},
        token::Token,
    },
};

mod elf;
mod parser;
mod riscv;
mod utils;

fn main() {
    let mut t = Token::lexer(include_str!("../main.S"));

    let nodes = nodes_from_tokens(&mut t);
    println!("{:#?}", nodes);
    let mut elf = Elf::new();
    let opcodes = riscv::encode_sections(nodes, &mut elf);

    elf.write(&Path::new("output.o"));
    println!("{:?}", opcodes);
}
