#![allow(non_snake_case)]

mod utils;

use jack_compiler::utils::{XmlWrite, init_writer};
use jack_compiler::{lexer, parser};
use std::{
    fs,
    io::{self, Cursor, Read, Write},
};

fn test_program(program_name: &str) -> io::Result<()> {
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
            let parser = parser::ClassParser::new();
            let ast = parser
                .parse(&source, lex)
                .unwrap_or_else(|e| panic!("error occurs while parsing: {:?}", e));

            // Create an in-memory buffer to simulate an on-disk file
            let buf = Cursor::new(Vec::new());
            let mut writer = init_writer(buf);
            if let Err(e) = ast.write_xml(&mut writer) {
                panic!("error occurs while writing XML: {}", e);
            }
            writer.inner_mut().flush()?;

            let buf_len = writer.inner_ref().get_ref().len();
            let mut out = String::with_capacity(buf_len);
            writer.inner_mut().read_to_string(&mut out)?;

            let class_name = path.file_stem().unwrap().to_str().unwrap();
            let cmp_path = format!("tests/programs/{}/{}.xml", program_name, class_name);
            let cmp = fs::read_to_string(&cmp_path)?;

            assert!(
                out.lines()
                    .zip(cmp.lines())
                    .all(|(out_line, cmp_line)| out_line == cmp_line)
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
