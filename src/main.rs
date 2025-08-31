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
    for node in nodes {
        let a = riscv::encode(node);

        println!("{:x}", u32::from_le_bytes(a[..4].try_into().unwrap()));
    }
}
