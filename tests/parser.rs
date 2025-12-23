#![allow(non_snake_case)]

mod utils;

use jack_compiler::utils::{XmlWrite, init_writer};
use jack_compiler::{lexer, parser};
use std::{
    fs,
    io::{self, Write},
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
            fs::create_dir_all(format!("tests/out/{}", program_name))?;
            let class_name = path.file_stem().unwrap().to_str().unwrap();
            let out_path = format!("tests/out/{}/{}.xml", program_name, class_name);
            let cmp_path = format!("tests/programs/{}/{}.xml", program_name, class_name);

            let source = fs::read_to_string(&path)?;
            let lex = lexer::Lexer::new(&source);
            let parser = parser::ClassParser::new();
            let ast = parser
                .parse(&source, lex)
                .unwrap_or_else(|e| panic!("error occurs while parsing: {:?}", e));

            let f = io::BufWriter::new(
                fs::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(&out_path)?,
            );
            let mut writer = init_writer(f);

            if let Err(e) = ast.write_xml(&mut writer) {
                panic!("Error: {}", e);
            }
            writer.inner_mut().flush()?;

            let out = fs::read_to_string(&out_path)?;
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
