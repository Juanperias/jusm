// fix this code

use logos::Logos;

use crate::{
    ast::{AstNode, nodes_from_tokens},
    token::Token,
};

mod ast;
mod riscv;
mod token;
mod utils;

fn main() {
    let mut t = Token::lexer("addi a0, zero, 10\necall");

    let nodes = nodes_from_tokens(&mut t);
    println!("{:?}", nodes);
}
