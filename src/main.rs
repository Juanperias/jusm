// fix this code

use logos::Logos;

use crate::parser::{
    ast::{AstNode, nodes_from_tokens},
    token::Token,
};

mod elf;
mod parser;
mod riscv;
mod utils;

fn main() {
    let mut t = Token::lexer(include_str!("../main.S"));

    let nodes = nodes_from_tokens(&mut t);
    println!("{:?}", nodes);
    let opcodes = riscv::encode_nodes(nodes);
    println!("{:?}", opcodes);
}
