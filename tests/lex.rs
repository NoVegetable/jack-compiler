#![allow(non_snake_case)]

mod utils;

use jack_compiler::{lexer, token};
use std::fs;
use std::io;
use utils::{keyword_to_literal, symbol_to_literal};

fn test_program(program_name: &str) -> io::Result<()> {
    use token::Token::*;

    let entries = fs::read_dir(format!("tests/programs/{}", program_name))?;

    // Loop over the files in the program directory
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(ext) = path.extension()
            && ext == "jack"
        {
            let source = fs::read_to_string(&path)?;
            let lex = lexer::Lexer::new(&source);

            let tokens = lex.map(|spanned| {
                let (_, token, _) = spanned.unwrap();
                match token {
                    Keyword(kw) => format!("<keyword> {} </keyword>", keyword_to_literal(&kw)),
                    // Turn "<" and ">" into "&lt;" and "&gt;" separately to match the compare files
                    Symbol(sym) => format!("<symbol> {} </symbol>", symbol_to_literal(&sym)),
                    // Remove the leading and trailing double quotes to match the compare files
                    StringConstant(str) => format!(
                        "<stringConstant> {} </stringConstant>",
                        &str[1..str.len() - 1]
                            .replace("<", "^lt;")
                            .replace(">", "&gt;")
                            .replace("&", "&amp;")
                    ),
                    IntegerConstant(int) => {
                        format!("<integerConstant> {} </integerConstant>", int)
                    }
                    Identifier(ident) => format!("<identifier> {} </identifier>", ident),
                }
            });

            let class_name = path.file_stem().unwrap().to_str().unwrap();
            let compare_str = fs::read_to_string(format!(
                "tests/programs/{}/{}T.xml",
                program_name, class_name
            ))?;
            assert!(
                tokens
                    .zip(compare_str.lines().skip(1)) // Skip the leading "<tokens>" line
                    .all(|(parsed, expected)| { parsed == expected })
            );
        }
    }

    Ok(())
}

#[test]
fn test_ArrayTest() {
    assert!(test_program("ArrayTest").is_ok());
}

#[test]
fn test_ExpressionLessSquare() {
    assert!(test_program("ExpressionLessSquare").is_ok());
}

#[test]
fn test_Square() {
    assert!(test_program("Square").is_ok());
}
