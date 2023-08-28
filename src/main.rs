use std::path::Path;

use clap::Parser;
use lib::{Lexer, Scanner};

mod prelude;
use prelude::*;

fn main() {
    startup_logger();

    let args = Args::parse();

    // Currently only supports one file
    let file = args.file;

    let scanner = Scanner::from_path(Path::new(&file)).expect("where file");
    let lexer = Lexer::new(scanner);

    lexer.for_each(|t| println!("{:?}", t));
    log::info!("Finished Compiling Succesfully");
}
