use clap::{Args, Parser};

#[derive(Parser)]
#[command(about = r#"Compiler Driver for "Writing a C Compiler" by Nora Sandler"#, long_about = None)]
struct Cli {
    /// The path to the C source file which should be compiled
    path_to_source: std::path::PathBuf,
    #[command(flatten)]
    args: Arguments,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct Arguments {
    /// Run the lexer, but stop before parsing
    #[arg(short, long)]
    lex: bool,

    /// Run the lexer and parser, but stop before assembly generation
    #[arg(short, long)]
    parse: bool,

    /// Perform lexing, parsing, and assembly generation, but stop before code emission
    #[arg(short, long)]
    codegen: bool,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    // Canonicalize actually accesses the filesystem to get the absolute path :(
    // I guess there's no better way to get the absolute path if someone
    // were to supply a path with ../../
    let absolute_path = std::fs::canonicalize(cli.path_to_source)?;

    let Arguments {
        lex,
        parse,
        codegen,
    } = cli.args;

    // This match could also be done with match cli.args
    // and then matching on Arguments {lex: true, parse: _, codegen: _},
    // but that is way to long and doesn't look good after rustfmt imho
    let result = match (lex, parse, codegen) {
        (true, _, _) => lex_file(absolute_path),
        (_, true, _) => lex_and_parse_file(absolute_path),
        (_, _, true) => lex_parse_and_codegen_file(absolute_path),
        _ => compile_file(absolute_path),
    };

    if result.is_err() {
        // As requested by the test runners, failed lexing/parsing/codegen should
        // result in a non-zero exit code
        std::process::exit(1)
    }

    Ok(())
}

fn lex_file(path: std::path::PathBuf) -> Result<(), String> {
    println!("Lexing {}", path.display());
    let mut lexer = lexer::Lexer::new(path)?;
    let _tokens = lexer.lex()?;

    // Parsing tokens did not fail, so yayyy
    Ok(())
}

fn lex_and_parse_file(path: std::path::PathBuf) -> Result<(), String> {
    println!("Lexing and parsing {}", path.display());
    let mut lexer = lexer::Lexer::new(path)?;
    let parser = parser::Parser::new();

    let tokens = lexer.lex()?;
    let _ast = parser.parse(&tokens)?;

    todo!()
}

fn lex_parse_and_codegen_file(path: std::path::PathBuf) -> Result<(), String> {
    println!("Lexing, parsing and codegen for {}", path.display());
    todo!()
}

fn compile_file(path: std::path::PathBuf) -> Result<(), String> {
    println!("Compiling {}", path.display());
    todo!()
}
