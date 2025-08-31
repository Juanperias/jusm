use super::token::Token;
use crate::utils::{check_is_reg, check_num, token_to_reg};
use logos::Lexer;

#[derive(Debug)]
pub enum AstNode {
    Addi { rd: u32, rs1: u32, imm: u64 },
    Ecall,
}

//TODO: improve this
pub fn nodes_from_tokens(lex: &mut Lexer<'_, Token>) -> Vec<AstNode> {
    let mut nodes = Vec::new();

    while let Some(token) = lex.next() {
        match token {
            Ok(t) => match t {
                Token::Addi => {
                    let rd = next_reg(lex);

                    let rs1 = next_reg(lex);

                    let imm = next_num(lex);

                    nodes.push(AstNode::Addi { rd, rs1, imm });
                }
                Token::Ecall => {
                    nodes.push(AstNode::Ecall);
                }
                _ => {}
            },
            Err(_) => {
                println!("{:?}", lex.slice());
            }
        }
    }

    nodes
}

pub fn next_reg(lex: &mut Lexer<'_, Token>) -> u32 {
    let reg = lex.next().unwrap().unwrap();

    assert!(check_is_reg(&reg), "Expected reg");

    token_to_reg(&reg)
}

pub fn next_num(lex: &mut Lexer<'_, Token>) -> u64 {
    let val = lex.next().unwrap().unwrap();

    check_num(&val)
}
