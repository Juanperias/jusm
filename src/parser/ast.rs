use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
};

use super::token::Token;
use crate::utils::{check_num, token_to_identifier, token_to_name, token_to_reg, token_to_string};
use colored::Colorize;
use logos::Lexer;
use thiserror::Error;

pub static LINE: AtomicU64 = AtomicU64::new(0);

pub static SUCCESS: AtomicBool = AtomicBool::new(true);

#[derive(Debug, Error)]
pub enum AstError {}

// thanks to: https://github.com/Brayan-724/amrisk, for the original macro

macro_rules! generate_nodes {
    (
        $(
            $name:ident => [$($arg:ident),*]
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone)]
        pub enum AstNode {
            $(
                $name {
                    $(
                        #[doc = stringify!($arg)]
                        $arg: generate_nodes!(@arg_ty $arg),
                    )*
                },
            )*
            Section { name: String, content: Vec<AstNode> },
            Label { name: String, content: Vec<AstNode> },
            Assci { seq: Vec<u8> },
        }


        pub fn nodes_from_tokens(lex: &mut Lexer<'_, Token>, source: String)
            -> (Vec<AstNode>, HashMap<String, SymbolInfo>) {

            let mut ctx = ParserCtx::new();

            while let Some(token) = lex.next() {
                let span = lex.span();
                let line = source[..span.start].chars().filter(|&c| c == '\n').count() + 1;
                LINE.store(line as u64, Ordering::SeqCst);

                match token {
                    Ok(t) => match t {
                        $(
                            Token::$name => {
                                $(
                                  let $arg = generate_nodes!(@fn_ty $arg, lex);
                                )*

                                ctx.push(AstNode::$name {
                                    $(
                                        $arg,
                                    )*
                                });
                             },
                        )*
                        Token::Global | Token::Globl => {
                            let name = next_identifier(lex);

                            ctx.set_visibility(name, Visibility::Global);
                        }
                        Token::Weak => {
                            let name = next_identifier(lex);

                            ctx.set_weakness(name, true);
                        }
                        Token::Nop => {
                            ctx.push(AstNode::Addi {
                                rd: 0,
                                rs1: 0,
                                imm: 0
                            });
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
                    },
                    },
                    Err(_) => {
                        SUCCESS.store(false, Ordering::SeqCst);
                        println!(
                            "{}:\n \tFound: {:?}\r\n\tLine: {}",
                            "Error, Invalid Token".bright_red(),
                            lex.slice(),
                            LINE.load(Ordering::Relaxed)
                        );

                    },
                }

            }

            ctx.push_label();

            if !SUCCESS.load(Ordering::Relaxed) {
                panic!("Invalid syntax");
            }

            ctx.get()
        }
    };


    (@fn_ty rd, $lex: expr) => { next_reg($lex) };
    (@fn_ty rs1, $lex: expr) => { next_reg($lex) };
    (@fn_ty rs2, $lex: expr) => { next_reg($lex) };
    (@fn_ty imm, $lex: expr) => { next_num($lex) };
    (@fn_ty symbol, $lex: expr) => { next_identifier($lex) };
    (@fn_ty paren_rs1, $lex: expr) => { next_in_paren($lex, next_reg) };

    (@arg_ty rd) => { u32 };
    (@arg_ty paren_rs1) => { u32 };
    (@arg_ty rs1) => { u32 };
    (@arg_ty rs2) => { u32 };
    (@arg_ty imm) => { u64 };
    (@arg_ty symbol) => { String };
}


// original macro by https://github.com/Brayan-724/amrisk
generate_nodes! {
    Addi => [rd, rs1, imm],
    Sub => [rd, rs1, rs2],
    Add => [rd, rs1, rs2],
    Xor => [rd, rs1, rs2],
    Sll => [rd, rs1, rs2],
    Srl => [rd, rs1, rs2],
    Sra => [rd, rs1, rs2],
    Slt => [rd, rs1, rs2],
    Sltu => [rd, rs1, rs2],

    La => [rd, symbol],

    Lui => [rd, imm],
    Auipc => [rd, imm],

    Sb => [rs2, imm, paren_rs1],

    Ecall => [],
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
    pub functions_info: HashMap<String, SymbolInfo>,
}

#[derive(Debug)]
pub struct SymbolInfo {
    pub visibility: Visibility,
    pub weak: bool,
}

impl Default for SymbolInfo {
    fn default() -> Self {
        Self {
            visibility: Visibility::Local,
            weak: false,
        }
    }
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
            functions_info: HashMap::new(),
        }
    }
    pub fn set_visibility(&mut self, name: String, visibility: Visibility) {
        if !self.functions_info.contains_key(&name) {
            self.functions_info.insert(
                name,
                SymbolInfo {
                    visibility,
                    ..Default::default()
                },
            );

            return;
        }

        self.functions_info.get_mut(&name).unwrap().visibility = visibility;
    }
    pub fn set_weakness(&mut self, name: String, weak: bool) {
        if !self.functions_info.contains_key(&name) {
            self.functions_info.insert(
                name,
                SymbolInfo {
                    weak,
                    ..Default::default()
                },
            );
            return;
        }

        self.functions_info.get_mut(&name).unwrap().weak = weak;
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
    pub fn get(mut self) -> (Vec<AstNode>, HashMap<String, SymbolInfo>) {
        if self.current_section.is_some() {
            let current = self.current_section.as_ref().unwrap();
            self.nodes.push(AstNode::Section {
                name: current.0.clone(),
                content: current.1.clone(),
            });
        }

        (self.nodes, self.functions_info)
    }
}
