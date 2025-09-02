use logos::Logos;

#[derive(Logos, Debug, PartialEq, PartialOrd, Ord, Eq)]
#[logos(skip r"[ \t\n\f,]+")]
pub enum Token {
    #[token("ecall")]
    Ecall,

    #[token("addi")]
    Addi,

    #[token("nop")]
    Nop,

    #[token("add")]
    Add,
        
    #[token("zero")]
    Zero,

    #[token("a7")]
    A7,

    #[token("a0")]
    A0,

    #[token("a1")]
    A1,

    #[token("a2")]
    A2,

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

    #[regex("#(.*)")]
    Comment,

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*:", |lex| {
        lex.slice().replace(":", "").to_string()
    })]
    Label(String),

    #[regex(r"\..*", |lex| lex.slice().to_string())]
    Name(String),
}
