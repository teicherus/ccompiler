use clap::{Args, Parser};

#[derive(Parser)]
#[command(about = r#"Compiler Driver for "Writing a C Compiler" by Nora Sandler"#, long_about = None)]
pub struct Cli {
    /// The path to the C source file which should be compiled
    path_to_source: std::path::PathBuf,
    #[command(flatten)]
    args: Arguments,
}

#[derive(Args)]
#[group(required = false, multiple = false)] // This makes the args mutually exclusive
pub struct Arguments {
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

/// The driver that handles lexing, parsing, codegen and compilation.
pub struct CompilerDriver {
    filepath: std::path::PathBuf,
    args: Arguments,
}

impl CompilerDriver {
    pub fn new() -> Result<Self, String> {
        let cli = Cli::parse();

        // Canonicalize actually accesses the filesystem to get the absolute path :(
        // I guess there's no better way to get the absolute path if someone
        // were to supply a path with ../../
        let absolute_path =
            std::fs::canonicalize(cli.path_to_source).map_err(|err| err.to_string())?;

        Ok(Self {
            filepath: absolute_path,
            args: cli.args,
        })
    }

    // Run either lexing, lexing and parsing,
    // lexing, parsing and codegen or a full compile.
    pub fn run(&self) -> Result<(), String> {
        match self.args {
            // Arguments are mutually exclusive thanks to clap, so we can ignore
            // all other args if one of them is true.
            Arguments { lex: true, .. } => self.lex_file(),
            Arguments { parse: true, .. } => self.lex_and_parse_file(),
            Arguments { codegen: true, .. } => self.lex_parse_and_codegen_file(),
            _ => self.compile_file(),
        }
    }

    fn lex_file(&self) -> Result<(), String> {
        println!("Lexing {}", self.filepath.display());
        let mut lexer = lexer::Lexer::new(self.filepath.clone())?;
        let _tokens = lexer.lex()?;

        // Parsing tokens did not fail, so yayyy
        Ok(())
    }

    fn lex_and_parse_file(&self) -> Result<(), String> {
        println!("Lexing and parsing {}", self.filepath.display());
        let mut lexer = lexer::Lexer::new(self.filepath.clone())?;
        let parser = parser::Parser::new();

        let tokens = lexer.lex()?;
        let _ast = parser.parse(&tokens)?;

        todo!()
    }

    fn lex_parse_and_codegen_file(&self) -> Result<(), String> {
        println!(
            "Lexing, parsing and codegen for {}",
            self.filepath.display()
        );
        todo!()
    }

    fn compile_file(&self) -> Result<(), String> {
        println!("Compiling {}", self.filepath.display());
        todo!()
    }
}
