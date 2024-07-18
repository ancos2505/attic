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
    // comm::print_ports();

    let port_path = "/dev/ttyS10";
    let baud_rate = "9600";

    comm::run(port_path, baud_rate)?;

    // comm::open_port("/dev/ttyS11")?;

    // comm::open_port("/dev/ttyS12")?;
    Ok(())
}
