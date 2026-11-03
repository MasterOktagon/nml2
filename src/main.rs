use std::{
    collections::HashMap,
    io::{IsTerminal, Read},
};

use ariadne::Fmt;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "NewGRF Meta Language compiler for OpenTTD", long_about = None)]
struct Args {
    pub file: Option<std::path::PathBuf>,

    #[arg(short, long)]
    pub debug: bool,
}

fn main() {
    let arguments = Args::parse();

    // read from source file if given, else read from stdin. this allows piping of stdin if you use that
    let (nml_source, file_name) = if let Some(file) = &arguments.file {
        (
            std::fs::read_to_string(&file)
                .expect(format!("could not open file: {}", file.display()).as_str()),
            file.to_str().unwrap(),
        )
    } else if !std::io::stdin().is_terminal() {
        let mut buf = "".to_string();
        std::io::stdin()
            .read_to_string(&mut buf)
            .expect("Could not read from stdin");
        (buf, "<stdin>")
    } else {
        println!(
            "{}: No input specified",
            "Error".fg(ariadne::Color::BrightRed)
        );
        return;
    };

    let mut codebase: HashMap<String, ariadne::Source> = HashMap::new();
    codebase.insert(
        file_name.to_string(),
        ariadne::Source::from(nml_source.clone()),
    );

    let parser = nmllib::parser::NmlParser::new();
    let ast = match parser.parse(file_name, &mut nmllib::reports::H(codebase.clone()), &nml_source) {
        Ok(ast) => ast,
        Err(err) => {
            match err {
                lalrpop_util::ParseError::InvalidToken { location } => {
                    todo!()
                }
                lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                    ariadne::Report::build(
                        ariadne::ReportKind::Error,
                        (file_name, token.0..token.2),
                    )
                    .with_message(format!("Unrecognized token: {}", token.1))
                    .with_note(format!("expected one of: {}", expected.join(", ")))
                    .finish()
                    .eprint(&mut nmllib::reports::H(codebase)).unwrap()
                },
                _ => {}
            }

            return;
        }
    };

    if arguments.debug {
        println!("ast = {:?}", ast)
    }

    // Do nice error reporting
    //for report in reports {
    //    report.print((input_file_name, ariadne::Source::from(nml_source)));
    //}
}
