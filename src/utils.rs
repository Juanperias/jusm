use crate::parser::token::Token;

pub fn check_num(reg: &Token) -> u64 {
    match reg {
        Token::Number(n) => *n,
        Token::NegNumber(n) => *n as u64,
        Token::Char(c) => {
            if !c.is_ascii() {
                panic!("Cannot use this char");
            }

            (*c as u8) as u64
        }
        _ => panic!("Expected number"),
    }
}

pub fn token_to_reg(token: &Token) -> u32 {
    match token {
        Token::Zero => 0,
        Token::A0 => 10,
        Token::A1 => 11,
        Token::A2 => 12,
        Token::A7 => 17,
        Token::A3 => 13,
        Token::Sp => 2,
        Token::T3 => 28,
        _ => panic!("Expected reg"),
    }
}

pub fn token_to_name(token: &Token) -> String {
    match token {
        Token::Name(s) => s.to_string(),
        _ => panic!("Expected name"),
    }
}
