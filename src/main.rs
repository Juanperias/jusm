// fix this code

use std::{
    cell::OnceCell,
    fs::{File, read_to_string},
    path::Path,
    sync::{Arc, Once},
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

    let nodes = nodes_from_tokens(&mut t, code.clone());
    println!("{:#?}", nodes);
    let elf = riscv::encode_sections(nodes.0, nodes.1);
    
    println!("{}", cli.output);
    elf.write(&Path::new(&cli.output));
}
