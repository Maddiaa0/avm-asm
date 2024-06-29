use avm_asm::compiler::compile_file;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(name = "avm-asm", version = "0.1.0", author = "Maddiaa")]
struct AvmAsm {
    pub path: Option<String>,
}

fn main() {
    let cli = AvmAsm::parse();

    // Check if no argument is provided
    if cli.path.is_none() {
        println!("No path provided! Use --help for more information.");
        return;
    }

    // Read the file
    let path = cli.path.unwrap();

    let bytecode = compile_file(&path);

    println!("{bytecode}");
}
