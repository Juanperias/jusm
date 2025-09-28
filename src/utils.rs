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
        Token::Zero | Token::X0 => 0,
        Token::X1 | Token::Ra => 1,
        Token::X2 | Token::Sp => 2,
        Token::X3 | Token::Gp => 3,
        Token::X4 | Token::Tp => 4,
        Token::X5 | Token::T0 => 5,
        Token::X6 | Token::T1 => 6,
        Token::X7 | Token::T2 => 7,
        Token::X8 | Token::S0 | Token::Fp => 8,
        Token::X9 | Token::S1 => 9,
        Token::X10 | Token::A0 => 10,
        Token::X11 | Token::A1 => 11,
        Token::X12 | Token::A2 => 12,
        Token::X13 | Token::A3 => 13,
        Token::X14 | Token::A4 => 14,
        Token::X15 | Token::A5 => 15,
        Token::X16 | Token::A6 => 16,
        Token::X17 | Token::A7 => 17,
        Token::X18 | Token::S2 => 18,
        Token::X19 | Token::S3 => 19,
        Token::X20 | Token::S4 => 20,
        Token::X21 | Token::S5 => 21,
        Token::X22 | Token::S6 => 22,
        Token::X23 | Token::S7 => 23,
        Token::X24 | Token::S8 => 24,
        Token::X25 | Token::S9 => 25,
        Token::X26 | Token::S10 => 26,
        Token::X27 | Token::S11 => 27,
        Token::X28 | Token::T3 => 28,
        Token::X29 | Token::T4 => 29,
        Token::X30 | Token::T5 => 30,
        Token::X31 | Token::T6 => 31,

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

pub fn token_to_identifier(token: &Token, lex: &mut Lexer<'_, Token>) -> String {
    match token {
        Token::Identifier(s) => s.to_string(),
        _ => {
            SUCCESS.store(false, Ordering::SeqCst);

            println!(
                "{}\n\tFound: {}\n\tLine: {}",
                "Syntax Error, Expected identifier:".bright_red(),
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
