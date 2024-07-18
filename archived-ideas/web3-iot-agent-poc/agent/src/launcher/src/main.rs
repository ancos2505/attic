use helpers::AppResult;

fn main() {
    let name = env!("PACKAGE_NAME");
    let version = env!("PACKAGE_VERSION");
    println!("{name} v{version}:");
    println!("Hello, world!");

    // 1. Load config from name.toml

    // 2. Load config from clap

    // 0. print serial ports
    if let Err(error) = rmain() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn rmain() -> AppResult<()> {
    core::run()?;
    Ok(())
}
