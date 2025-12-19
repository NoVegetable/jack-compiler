#![allow(non_snake_case)]

use jack_compiler::lexer;
use std::fs;
use std::io;

fn test_program(program_name: &str) -> io::Result<()> {
    use lexer::Token::*;

    let entries = fs::read_dir(format!("tests/programs/{}", program_name))?;

    // Loop over the files in the program directory
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(ext) = path.extension()
            && ext == "jack"
        {
            println!("testing {}", path.file_name().unwrap().display());

            let source = fs::read_to_string(&path)?;
            let lex = lexer::lexer(&source);

            let tokens = lex.map(|t| match t.unwrap() {
                Keyword(kw) => format!("<keyword> {} </keyword>", kw),
                // Turn "<" and ">" into "&lt;" and "&gt;" separately to match the compare files
                Symbol(symbol) => match symbol {
                    "<" => format!("<symbol> &lt; </symbol>"),
                    ">" => format!("<symbol> &gt; </symbol>"),
                    "&" => format!("<symbol> &amp; </symbol>"),
                    _ => format!("<symbol> {} </symbol>", symbol),
                },
                // Remove the leading and trailing double quotes to match the compare files
                StringConstant(str) => format!(
                    "<stringConstant> {} </stringConstant>",
                    &str[1..str.len() - 1]
                        .replace("<", "^lt;")
                        .replace(">", "&gt;")
                        .replace("&", "&amp;")
                ),
                IntegerConstant(int) => format!("<integerConstant> {} </integerConstant>", int),
                Identifier(ident) => format!("<identifier> {} </identifier>", ident),
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
