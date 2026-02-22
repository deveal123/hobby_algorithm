mod bundler;
mod resolver;

use crate::bundler::Bundler;
use crate::resolver::ModuleIndex;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // Assume we run from workspace root or find it
    // For simplicity, assume current dir is workspace root or one level down?
    // Let's assume we run from 'd:\hobby\algorithm'
    let root_dir = std::env::current_dir().unwrap();

    let index = ModuleIndex::new(&root_dir);
    let bundler = Bundler::new(index);

    let bundled_code = bundler.bundle(&args.input);

    if let Some(out_path) = args.output {
        std::fs::write(out_path, bundled_code).expect("Failed to write output");
    } else {
        println!("{}", bundled_code);
    }
}
