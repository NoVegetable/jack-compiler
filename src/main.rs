use jack_compiler::lexer;
use jack_compiler::parser;
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        panic!("Please provide Jack source file(s).\n");
    }

    for arg in &args[1..] {
        let source = fs::read_to_string(arg)?;
        let lexer = lexer::Lexer::new(&source);
        let parser = parser::ClassParser::new();
        let ast = parser
            .parse(lexer)
            .unwrap_or_else(|e| panic!("error occurs while parsing: {:?}", e));
        println!("{:#?}", ast);
    }

    Ok(())
}
