use logos::Logos;

#[derive(Logos, Debug, PartialEq, PartialOrd, Ord, Eq)]
#[logos(skip r"[ \t\n\f,]+")]
pub enum Token {
    #[token(".ascii")]
    Assci,

    #[token(".section")]
    Section,

    #[token("ecall")]
    Ecall,

    #[token("auipc")]
    Auipc,

    #[token("lui")]
    Lui,

    #[token("addi")]
    Addi,

    #[token("nop")]
    Nop,

    #[token("add")]
    Add,

    #[token("sub")]
    Sub,

    #[token("mv")]
    Mv,

    #[token("xor")]
    Xor,

    #[token("or")]
    Or,

    #[token("and")]
    And,

    #[token("sll")]
    Sll,

    #[token("srl")]
    Srl,

    #[token("sra")]
    Sra,

    #[token("slt")]
    Slt,

    #[token("sltu")]
    Sltu,

    #[token("zero")]
    Zero,

    #[token("a7")]
    A7,

    #[token("sp")]
    Sp,

    #[token("t3")]
    T3,

    #[token("(")]
    ParenthesisStart,

    #[token(")")]
    ParenthesisEnd,

    #[token("a0")]
    A0,

    #[token("a1")]
    A1,

    #[token("a2")]
    A2,

    #[token("a3")]
    A3,

    #[token("sb")]
    Sb,

    #[regex(r"\d+",  |lex| {
          lex.slice().trim().parse::<u64>().expect("Invalid number")
    })]
    Number(u64),

    #[regex(r"-\d+", |lex| {
        lex.slice().trim().parse::<i64>().expect("Invalid number")
    })]
    NegNumber(i64),

    #[regex(r"'(\\.|[A-Za-z])'", |lex| {
        let a = lex.slice().trim_matches('\'');
        let b = match a {
            r"\n" => "\n",
            s => s,
        };
        b.parse::<char>().unwrap()
    })]
    Char(char),

    #[regex("#(.*)")]
    Comment,

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*:", |lex| {
        lex.slice().replace(":", "").to_string()
    })]
    Label(String),

    #[regex(r#""[A-Za-z_][A-Za-z0-9_]*""#, |lex| {
        lex.slice().trim_matches('"').to_string()
    })]
    Str(String),

    #[regex(r"\.[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_string())]
    Name(String),

    Empty,
}

impl Default for Token {
    fn default() -> Self {
        Self::Empty
    }
}
