use logos::Logos;


#[derive(Logos, Debug, PartialEq, PartialOrd, Ord, Eq)]
#[logos(skip r"[ \t\n\f,]+")]
pub enum Token {
    // Ins
    //
    // I Type
    
    #[token("addi")]
    Addi,

    #[token("ecall")]
    Ecall,


    // R Type
    //

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

    // S type
    #[token("sb")]
    Sb,

    // U type

    #[token("auipc")]
    Auipc,

    #[token("lui")]
    Lui,

    // Pseudo
    #[token("la")]
    La,

    #[token("nop")]
    Nop,

    // Regs
    #[token("x0")]
    X0,

    #[token("zero")]
    Zero,

    #[token("x1")]
    X1,

    #[token("ra")]
    Ra,

    #[token("x2")]
    X2,

    #[token("sp")]
    Sp,

    #[token("x3")]
    X3,

    #[token("gp")]
    Gp,

    #[token("x4")]
    X4,

    #[token("tp")]
    Tp,

    #[token("x5")]
    X5,

    #[token("t0")]
    T0,

    #[token("x6")]
    X6,

    #[token("t1")]
    T1,

    #[token("x7")]
    X7,

    #[token("t2")]
    T2,

    #[token("x8")]
    X8,

    #[token("s0")]
    S0,

    #[token("fp")]
    Fp,

    #[token("x9")]
    X9,

    #[token("s1")]
    S1,

    #[token("x10")]
    X10,

    #[token("a0")]
    A0,

    #[token("x11")]
    X11,

    #[token("a1")]
    A1,

    #[token("x12")]
    X12,

    #[token("a2")]
    A2,

    #[token("x13")]
    X13,

    #[token("a3")]
    A3,

    #[token("x14")]
    X14,

    #[token("a4")]
    A4,

    #[token("x15")]
    X15,

    #[token("a5")]
    A5,

    #[token("x16")]
    X16,

    #[token("a6")]
    A6,

    #[token("x17")]
    X17,

    #[token("a7")]
    A7,

    #[token("x18")]
    X18,

    #[token("s2")]
    S2,

    #[token("x19")]
    X19,

    #[token("s3")]
    S3,

    #[token("x20")]
    X20,

    #[token("s4")]
    S4,

    #[token("x21")]
    X21,

    #[token("s5")]
    S5,

    #[token("x22")]
    X22,

    #[token("s6")]
    S6,

    #[token("x23")]
    X23,

    #[token("s7")]
    S7,
    
    #[token("x24")]
    X24,

    #[token("s8")]
    S8,

    #[token("x25")]
    X25,

    #[token("s9")]
    S9,

    #[token("x26")]
    X26,

    #[token("s10")]
    S10,
    
    #[token("x27")]
    X27,

    #[token("s11")]
    S11,

    #[token("x28")]
    X28,

    #[token("t3")]
    T3,

    #[token("x29")]
    X29,

    #[token("t4")]
    T4,

    #[token("x30")]
    X30,

    #[token("t5")]
    T5,

    #[token("x31")]
    X31,

    #[token("t6")]
    T6,

    // Data Types
    #[token(".ascii")]
    Assci,

      
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
            r"\r" => "\r",
            r"\t" => "\t",
            s => s,
        };
        b.parse::<char>().unwrap()
    })]
    Char(char),

    #[regex(r#"[A-Za-z_][A-Za-z0-9_]*""#, |lex| {
        lex.slice().trim_matches('"').to_string()
    })]
    Str(String),




    // ELF
    #[token(".section")]
    Section,

    #[token(".global")]
    Global,

    #[token(".weak")]
    Weak,

    #[token(".globl")]
    Globl,

    // assembly
    #[regex(r"[A-Za-z_][A-Za-z0-9_]*:", |lex| {
        lex.slice().replace(":", "").to_string()
    })]
    Label(String),

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", |lex| {
        lex.slice().replace(":", "").to_string()
    })]
    Identifier(String),

    #[regex(r"\.[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_string())]
    Name(String),

    // symbols
    #[token("(")]
    ParenthesisStart,

    #[token(")")]
    ParenthesisEnd,

    #[regex("#(.*)")]
    Comment,



    Empty,
}

  
impl Default for Token {
    fn default() -> Self {
        Self::Empty
    }
}
