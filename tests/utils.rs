#![allow(dead_code)]

use jack_compiler::token::{Keyword, Symbol};

pub fn keyword_to_literal(kw: &Keyword) -> &'static str {
    use Keyword::*;

    match *kw {
        Class => "class",
        Constructor => "constructor",
        Function => "function",
        Method => "method",
        Field => "field",
        Static => "static",
        Var => "var",
        Int => "int",
        Char => "char",
        Boolean => "boolean",
        Void => "void",
        True => "true",
        False => "false",
        Null => "null",
        This => "this",
        Let => "let",
        Do => "do",
        If => "if",
        Else => "else",
        While => "while",
        Return => "return",
    }
}

pub fn symbol_to_literal(sym: &Symbol) -> &'static str {
    use Symbol::*;

    match *sym {
        OpenBrace => "{",
        CloseBrace => "}",
        OpenParen => "(",
        CloseParen => ")",
        OpenBracket => "[",
        CloseBracket => "]",
        Dot => ".",
        Comma => ",",
        Semi => ";",
        Plus => "+",
        Minus => "-",
        Star => "*",
        Slash => "/",
        And => "&amp;",
        Or => "|",
        Lt => "&lt;",
        Gt => "&gt;",
        Eq => "=",
        Tilde => "~",
    }
}
