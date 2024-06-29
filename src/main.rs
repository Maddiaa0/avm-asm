use clap::{App, Parser};

use avm_asm::compiler::compile_asm;

#[derive(Parser, Debug, Clone)]
#[clap(name = "avm-asm", version = "0.1.0", author = "Maddiaa0")]
struct AvmAsm {
    pub path: Option<String>,
}

fn main() {
    let mut cli = AvmAsm::parse();

    // Check if no argument is provided
    if cli.path.is_none() {
        println!("No path provided! Use --help for more information.");
        return;
    }

    // Read the file
    let path = cli.path.unwrap();
    let input = std::fs::read_to_string(path).unwrap();

    let bytecode = compile_asm(input);

    println!("{bytecode}");
}