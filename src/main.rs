// fix this code

use std::{
    fs::{File, read_to_string},
    path::Path,
};

use logos::Logos;

use crate::parser::{ast::nodes_from_tokens, token::Token};
use clap::Parser;

mod elf;
mod parser;
mod riscv;
mod utils;

#[derive(Parser)]
pub struct Cli {
    pub file: String,

    #[clap(short, long)]
    pub output: String,
}

fn main() {
    let cli = Cli::parse();

    let code = read_to_string(cli.file).unwrap();

    let mut t = Token::lexer(&code);

    let nodes = nodes_from_tokens(&mut t);
    println!("{:#?}", nodes);
    let elf = riscv::encode_sections(nodes);

    println!("{}", cli.output);
    elf.write(&Path::new(&cli.output));
}
