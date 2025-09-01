use super::token::Token;
use crate::utils::{check_num, token_to_name, token_to_reg};
use logos::Lexer;

#[derive(Debug, Clone)]
pub enum AstNode {
    Section { name: String, content: Vec<AstNode> },
    Addi { rd: u32, rs1: u32, imm: u64 },
    Ecall,
}

pub struct ParserCtx {
    pub nodes: Vec<AstNode>,
    pub current_section: Option<(String, Vec<AstNode>)>,
}

impl ParserCtx {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            current_section: None,
        }
    }
    pub fn push(&mut self, node: AstNode) {
        if self.current_section.is_some() {
            self.current_section.as_mut().unwrap().1.push(node);
            return;
        }

        self.nodes.push(node);
    }
    pub fn get(&mut self) -> &Vec<AstNode> {
         if self.current_section.is_some() {
        let current = self.current_section.as_ref().unwrap();
        self.nodes.push(AstNode::Section {
            name: current.0.clone(),
            content: current.1.clone(),
        });
    }

         self.nodes.as_ref()
    }
}

//TODO: improve this
pub fn nodes_from_tokens(lex: &mut Lexer<'_, Token>) -> Vec<AstNode> {
    let mut ctx = ParserCtx::new();

    while let Some(token) = lex.next() {
        println!("{:?}", token);
        match token {
            Ok(t) => match t {
                Token::Addi => {
                    let rd = next_reg(lex);

                    let rs1 = next_reg(lex);

                    let imm = next_num(lex);

                    ctx.push(AstNode::Addi { rd, rs1, imm });
                }
                Token::Ecall => {
                    ctx.push(AstNode::Ecall);
                }
                Token::Section => {
                    if ctx.current_section.is_some() {
                        let current = ctx.current_section.unwrap();
                        ctx.nodes.push(AstNode::Section {
                            name: current.0,
                            content: current.1,
                        });
                    }

                    let name = next_name(lex);

                    ctx.current_section = Some((name, Vec::new()));
                }
                _ => {}
            },
            Err(_) => {
                println!("{:?}", lex.slice());
            }
        }
    }

    ctx.get().clone() 
}

pub fn next_reg(lex: &mut Lexer<'_, Token>) -> u32 {
    let reg = lex.next().unwrap().unwrap();

    token_to_reg(&reg)
}

pub fn next_name(lex: &mut Lexer<'_, Token>) -> String {
    let name = lex.next().unwrap().unwrap();

    token_to_name(&name)
}

pub fn next_num(lex: &mut Lexer<'_, Token>) -> u64 {
    let val = lex.next().unwrap().unwrap();

    check_num(&val)
}
