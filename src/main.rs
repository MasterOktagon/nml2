use std::io::Read;

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
    let nml_source = if let Some(file) = arguments.file {
        std::fs::read_to_string(&file)
            .expect(format!("could not open file: {}", file.display()).as_str())
    } else {
        let mut buf = "".to_string();
        std::io::stdin().read_to_string(&mut buf).expect("Could not read from stdin");

        buf
    };

    let parser = nmllib::parser::NmlParser::new();
    let ast = parser.parse(&nml_source).unwrap();

    if arguments.debug {
        println!("ast = {:?}", ast)
    }
}
