use logos::Logos;

#[derive(Logos, Debug, PartialEq, PartialOrd, Ord, Eq)]
#[logos(skip r"[ \t\n\f,]+")]
pub enum Token {
    #[token("ecall")]
    Ecall,

    #[token("addi")]
    Addi,

    #[token("zero")]
    Zero,

    #[token("a0")]
    A0,

    #[regex(r"\d+",  |lex| {
          lex.slice().trim().parse::<u64>().expect("Invalid number")
    })]
    Number(u64),

    #[regex(r"-\d+", |lex| {
        lex.slice().trim().parse::<i64>().expect("Invalid number")
    })]
    NegNumber(i64),

    #[token("section")]
    Section,

    // i think this is broken
    #[regex(";(.*)")]
    Comment,

    #[regex(r"\..*", |lex| lex.slice().to_string())]
    Name(String),
}
