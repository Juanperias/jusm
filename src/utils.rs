use std::sync::atomic::Ordering;

use colored::Colorize;
use logos::Lexer;

use crate::parser::{
    ast::{LINE, SUCCESS},
    token::Token,
};

pub fn check_num(reg: &Token, lex: &mut Lexer<'_, Token>) -> u64 {
    match reg {
        Token::Number(n) => *n,
        Token::NegNumber(n) => *n as u64,
        Token::Char(c) => {
            if !c.is_ascii() {
                panic!("Cannot use this char");
            }

            (*c as u8) as u64
        }
        _ => {
            SUCCESS.store(false, Ordering::SeqCst);

            println!(
                "{}\n\tFound: {}\n\tLine: {}",
                "Syntax Error, Expected number:".bright_red(),
                lex.slice(),
                LINE.load(Ordering::Relaxed)
            );

            0
        }
    }
}

pub fn token_to_reg(token: &Token, lex: &mut Lexer<'_, Token>) -> u32 {
    match token {
        Token::Zero => 0,
        Token::A0 => 10,
        Token::A1 => 11,
        Token::A2 => 12,
        Token::A7 => 17,
        Token::A3 => 13,
        Token::Sp => 2,
        Token::T3 => 28,
        _ => {
            SUCCESS.store(false, Ordering::SeqCst);

            println!(
                "{}\n\tFound: {}\n\tLine: {}",
                "Syntax Error, Expected Reg:".bright_red(),
                lex.slice(),
                LINE.load(Ordering::Relaxed)
            );

            0
        }
    }
}

pub fn token_to_name(token: &Token, lex: &mut Lexer<'_, Token>) -> String {
    match token {
        Token::Name(s) => s.to_string(),
        _ => {
            SUCCESS.store(false, Ordering::SeqCst);

            println!(
                "{}\n\tFound: {}\n\tLine: {}",
                "Syntax Error, Expected name:".bright_red(),
                lex.slice(),
                LINE.load(Ordering::Relaxed)
            );

            String::new()
        }
    }
}

pub fn token_to_string(token: &Token, lex: &mut Lexer<'_, Token>) -> String {
    match token {
        Token::Str(s) => s.to_string(),
        _ => {
            SUCCESS.store(false, Ordering::SeqCst);

            println!(
                "{}\n\tFound: {}\n\tLine: {}",
                "Syntax Error, Expected string:".bright_red(),
                lex.slice(),
                LINE.load(Ordering::Relaxed)
            );

            String::new()
        }
    }
}
