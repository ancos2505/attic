use clap::Parser;
use sri_hasher::{result::LibResult, sri_hasher::SriHasher};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Args {
    #[clap(long, default_value = "false")]
    quiet: bool,
    file_path: PathBuf,
}

fn main() {
    use std::ops::Not;
    let args = Args::parse();
    let is_quiet = args.quiet;

    if let Err(error) = rmain(args) {
        if is_quiet.not() {
            eprintln!("Error: {error}");
        }
        std::process::exit(1);
    }
}

fn rmain(args: Args) -> LibResult<()> {
    let output = SriHasher::new()
        .read_file(args.file_path)?
        .sha_256()?
        .base64_encode()?;
    println!("{output}");
    Ok(())
}
