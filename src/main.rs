use clap::{Arg, Command};
use jack_compiler::utils::XmlWrite;
use jack_compiler::{lexer, parser, utils};
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() -> io::Result<()> {
    let matches = Command::new("jack-compiler")
        .about("Jack compiler frontend")
        .arg(Arg::new("output").short('o').long("output").help(
            "The output path for the generated AST. If not set, the output would be set to stdout.",
        ))
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_parser(["xml", "debug"])
                .default_value("xml")
                .long_help(
"The output format. Possible values are: 'xml', 'debug'. They correspond to the 2 formats we
supports: XML and Rust debug print. The default value is 'xml'."
                ),
        )
        .arg(
            Arg::new("input")
                .help("The input Jack source file.")
                .required(true)
        )
        .get_matches();

    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output");
    let format = matches.get_one::<String>("format").unwrap();

    let source = fs::read_to_string(input)?;

    let lexer = lexer::Lexer::new(&source);
    let parser = parser::ClassParser::new();
    let ast = parser
        .parse(&source, lexer)
        .unwrap_or_else(|e| panic!("error occurs while parsing: {:?}", e));

    if let Some(out_path) = output {
        ensure_parent(out_path)?;

        let mut out = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(out_path)?;

        if format == "xml" {
            let mut writer = utils::init_writer(out);
            if let Err(e) = ast.write_xml(&mut writer) {
                panic!("error occurs while writing output: {}", e);
            }
        } else {
            out.write(format!("{:#?}", ast).as_bytes())?;
        }
    } else {
        let out = io::stdout();
        if format == "xml" {
            let mut writer = utils::init_writer(out);
            if let Err(e) = ast.write_xml(&mut writer) {
                panic!("error occurs while writing output: {}", e);
            }
        } else {
            print!("{:#?}", ast);
        }
    }

    Ok(())
}

fn ensure_parent<S: AsRef<OsStr> + ?Sized>(s: &S) -> io::Result<()> {
    let path = Path::new(s);
    let Some(parent) = path.parent() else {
        panic!("invalid output path");
    };
    fs::create_dir_all(parent)?;
    Ok(())
}
