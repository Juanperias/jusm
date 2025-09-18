use std::{collections::HashMap, sync::atomic::{AtomicBool, AtomicU64, Ordering}};

use super::token::Token;
use crate::utils::{check_num, token_to_identifier, token_to_name, token_to_reg, token_to_string};
use colored::Colorize;
use logos::Lexer;
use thiserror::Error;

pub static LINE: AtomicU64 = AtomicU64::new(0);

pub static SUCCESS: AtomicBool = AtomicBool::new(true);

#[derive(Debug, Error)]
pub enum AstError {}

#[derive(Debug, Clone)]
pub enum AstNode {
    Section { name: String, content: Vec<AstNode> },
    Label { name: String, content: Vec<AstNode> },
    Addi { rd: u32, rs1: u32, imm: u64 },
    Add { rd: u32, rs1: u32, rs2: u32 },
    Sub { rd: u32, rs1: u32, rs2: u32 },
    Xor { rd: u32, rs1: u32, rs2: u32 },
    Sll { rd: u32, rs1: u32, rs2: u32 },
    Srl { rd: u32, rs1: u32, rs2: u32 },
    Sra { rd: u32, rs1: u32, rs2: u32 },
    Slt { rd: u32, rs1: u32, rs2: u32 },
    Sltu { rd: u32, rs1: u32, rs2: u32 },
    Sb { rs1: u32, rs2: u32, imm: u64 },
    Lui { rd: u32, imm: u64 },
    Auipc { rd: u32, imm: u64 },
    La { rd: u32, symbol: String },
    Assci { seq: Vec<u8> },
    Ecall,
}

macro_rules! register_fn {
    ($name:ident, $ctx:expr, $lex:expr) => {{
        let (rd, rs1, rs2) = register_args($lex);
        $ctx.push(AstNode::$name { rd, rs1, rs2 });
    }};
}

macro_rules! upper_fn {
    ($name: ident, $ctx: expr, $lex: expr) => {{
        let (rd, imm) = upper_args($lex);
        $ctx.push(AstNode::$name { rd, imm });
    }};
}


pub fn nodes_from_tokens(lex: &mut Lexer<'_, Token>, source: String) -> (Vec<AstNode>, HashMap<String, Visibility>) {
    let mut ctx = ParserCtx::new();

    while let Some(token) = lex.next() {
        let span = lex.span();
        let line = source[..span.start].chars().filter(|&c| c == '\n').count() + 1;
        LINE.store(line as u64, Ordering::SeqCst);
        println!("{:?}", token);

        match token {
            Ok(t) => match t {
                Token::Addi => {
                    let rd = next_reg(lex);

                    let rs1 = next_reg(lex);

                    let imm = next_num(lex);

                    ctx.push(AstNode::Addi { rd, rs1, imm });
                }
                Token::Ecall => ctx.push(AstNode::Ecall),

                Token::Nop => {
                    ctx.push(AstNode::Addi {
                        rd: 0,
                        rs1: 0,
                        imm: 0,
                    });
                }
                Token::Global | Token::Globl => {
                    let name = next_identifier(lex);

                    ctx.push_visibility(name, Visibility::Global);
                }
                Token::La => {
                    let rd = next_reg(lex);
                    let symbol = next_identifier(lex);

                    ctx.push(AstNode::La { rd, symbol });
                }
                Token::Sub => register_fn!(Sub, ctx, lex),
                Token::Add => register_fn!(Add, ctx, lex),
                Token::Xor => register_fn!(Xor, ctx, lex),
                Token::Sll => register_fn!(Sll, ctx, lex),
                Token::Srl => register_fn!(Srl, ctx, lex),
                Token::Sra => register_fn!(Sra, ctx, lex),
                Token::Slt => register_fn!(Slt, ctx, lex),
                Token::Sltu => register_fn!(Sltu, ctx, lex),

                Token::Lui => upper_fn!(Lui, ctx, lex),
                Token::Auipc => upper_fn!(Auipc, ctx, lex),

                Token::Sb => {
                    let rs2 = next_reg(lex);
                    let imm = next_num(lex);

                    let rs1 = next_in_paren(lex, next_reg);

                    ctx.push(AstNode::Sb { rs2, rs1, imm });
                }
                Token::Label(s) => {
                    ctx.push_label();

                    ctx.current_label = Some((s, Vec::new()));
                }
                Token::Section => {
                    if ctx.current_section.is_some() {
                        ctx.push_label();
                        let current = ctx.current_section.unwrap();
                        ctx.nodes.push(AstNode::Section {
                            name: current.0,
                            content: current.1,
                        });
                    }

                    let name = next_name(lex);

                    ctx.current_section = Some((name, Vec::new()));
                }
                Token::Assci => {
                    let assci = next_string(lex);

                    ctx.push(AstNode::Assci {
                        seq: assci.into_bytes(),
                    });
                }
                Token::Comment => {}
                _ => {
                    SUCCESS.store(false, Ordering::SeqCst);
                    println!(
                        "{}:\n \tFound: {:?}\r\n\tLine: {}",
                        "Error, Unexpected token".bright_red(),
                        lex.slice(),
                        LINE.load(Ordering::Relaxed)
                    );
                }
            },
            Err(_) => {
                SUCCESS.store(false, Ordering::SeqCst);
                println!(
                    "{}:\n \tFound: {:?}\r\n\tLine: {}",
                    "Error, Invalid Token".bright_red(),
                    lex.slice(),
                    LINE.load(Ordering::Relaxed)
                );
            }
        }
    }
    
    ctx.push_label();

    if !SUCCESS.load(Ordering::Relaxed) {
        panic!("Invalid syntax");
    }

    ctx.get().clone()
}

pub fn register_args(lex: &mut Lexer<'_, Token>) -> (u32, u32, u32) {
    let rd = next_reg(lex);
    let rs1 = next_reg(lex);
    let rs2 = next_reg(lex);

    (rd, rs1, rs2)
}

pub fn upper_args(lex: &mut Lexer<'_, Token>) -> (u32, u64) {
    let rd = next_reg(lex);
    let imm = next_num(lex);

    (rd, imm)
}

pub fn next_string(lex: &mut Lexer<'_, Token>) -> String {
    let s = lex.next().unwrap().unwrap_or_default();

    token_to_string(&s, lex)
}

pub fn next_in_paren<T, F>(lex: &mut Lexer<'_, Token>, func: F) -> T
where
    F: Fn(&mut Lexer<'_, Token>) -> T,
{
    expect_token(lex, Token::ParenthesisStart);
    let ret = func(lex);
    expect_token(lex, Token::ParenthesisEnd);

    ret
}

pub fn expect_token(lex: &mut Lexer<'_, Token>, expected: Token) {
    let l = lex.next().unwrap().unwrap_or_default();

    if l != expected {
        println!(
            "{}\n\tExpected {:?}\n\tFound {:?}\n\tLine {}",
            "Syntax Error, Unexpected token:".bright_red(),
            expected,
            l,
            LINE.load(Ordering::Relaxed)
        );
    }
}

pub fn next_reg(lex: &mut Lexer<'_, Token>) -> u32 {
    let reg = lex.next().unwrap().unwrap_or_default();

    token_to_reg(&reg, lex)
}

pub fn next_name(lex: &mut Lexer<'_, Token>) -> String {
    let name = lex.next().unwrap().unwrap_or_default();

    token_to_name(&name, lex)
}

pub fn next_identifier(lex: &mut Lexer<'_, Token>) -> String {
    let ident = lex.next().unwrap().unwrap_or_default();

    token_to_identifier(&ident, lex)
}

pub fn next_num(lex: &mut Lexer<'_, Token>) -> u64 {
    let val = lex.next().unwrap().unwrap_or_default();

    check_num(&val, lex)
}

pub struct ParserCtx {
    pub nodes: Vec<AstNode>,
    pub current_section: Option<(String, Vec<AstNode>)>,
    pub current_label: Option<(String, Vec<AstNode>)>,
    pub functions_visibility: HashMap<String, Visibility>
}

#[derive(Clone, Copy, Debug)]
pub enum Visibility {
    Global,
    Local,
}

impl ParserCtx {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            current_section: None,
            current_label: None,
            functions_visibility: HashMap::new(),
        }
    }
    pub fn push_visibility(&mut self, name: String, visibility: Visibility) {
        self.functions_visibility.insert(name, visibility);
    }
    pub fn push(&mut self, node: AstNode) {
        if self.current_label.is_some() {
            self.current_label.as_mut().unwrap().1.push(node);
            return;
        } else if self.current_section.is_some() {
            self.current_section.as_mut().unwrap().1.push(node);

            return;
        }

        self.nodes.push(node);
    }
    pub fn push_label(&mut self) {
        if self.current_label.is_some() {
            let curr = self.current_label.clone().unwrap();

            self.current_label = None;

            self.push(AstNode::Label {
                name: curr.0.clone(),
                content: curr.1.clone(),
            });
        }
    }
    pub fn get(mut self) -> (Vec<AstNode>, HashMap<String, Visibility>) {
        if self.current_section.is_some() {
            let current = self.current_section.as_ref().unwrap();
            self.nodes.push(AstNode::Section {
                name: current.0.clone(),
                content: current.1.clone(),
            });
        }

        (self.nodes, self.functions_visibility)
    }
}
