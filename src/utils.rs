use std::ops::RangeBounds;

use crate::parser::token::Token;

pub fn check_is_reg(reg: &Token) -> bool {
    (Token::Zero..=Token::A0).contains(reg)
}

pub fn check_num(reg: &Token) -> u64 {
    match reg {
        Token::Number(n) => *n,
        Token::NegNumber(n) => *n as u64,
        _ => panic!("Expected number"),
    }
}

pub fn token_to_reg(token: &Token) -> u32 {
    match token {
        Token::Zero => 0,
        Token::A0 => 10,
        _ => 0,
    }
}
