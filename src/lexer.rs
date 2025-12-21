use crate::token::{LexicalError, Token};
use logos::{Logos, SpannedIter};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'source> {
    token_stream: SpannedIter<'source, Token>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            token_stream: Token::lexer(source).spanned(),
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}
