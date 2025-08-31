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
    let mut t = Token::lexer("addi a0, zero, 10\necall");

    let nodes = nodes_from_tokens(&mut t);
    println!("{:?}", nodes);
}
