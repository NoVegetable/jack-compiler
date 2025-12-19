use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"(?m)//.*?$|/\*[^*]*\*+(?:[^/*][^*]*\*+)*/|[ \t\n\r\f]+")]
pub enum Token<'a> {
    #[regex(
        "class|constructor|function|method|field|static|var|int|char|boolean|void|true|false|null|this|let|do|if|else|while|return",
        |lex| lex.slice()
    )]
    Keyword(&'a str),

    #[regex(r"[{}()\[\].,;+*\/&|<>=~-]", |lex| lex.slice())]
    Symbol(&'a str),

    #[regex(r#""[^"\n]*""#, |lex| lex.slice())]
    StringConstant(&'a str),

    #[regex(r"[0-9]|[1-9][0-9]|[1-9][0-9]{2}|[1-9][0-9]{3}|[1-2][0-9]{4}|3[0-1][0-9]{3}|32[0-6][0-9]{2}|327[0-5][0-9]|3276[0-7]", |lex| lex.slice().parse::<u16>().unwrap())]
    IntegerConstant(u16),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Identifier(&'a str),
}

#[inline]
pub fn lexer<'source>(
    source: &'source <Token<'source> as Logos<'source>>::Source,
) -> Lexer<'source, Token<'source>> {
    Token::lexer(&source)
}
