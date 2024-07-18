use clap::Parser;
use crossbeam_channel::Receiver;
use helpers::AppResult;

#[derive(Parser, Debug)] // requires `derive` feature
struct AppConfig {
    ///  Valid baud rates: 9600, 19200, 38400, 57600, or 115200.
    #[clap(short = 'r', long = "baud-rate", value_name = "9600")]
    baud_rate: Option<u32>, // MAX 115_200

    /// Serial port local path
    #[clap(short = 'p', long = "port-path", value_name = "/dev/ttyS11", value_hint = clap::ValueHint::FilePath)]
    port_path: Option<std::path::PathBuf>,
}

struct ThreadChannels {
    ctrl_c: Receiver<()>,
    agent_sc: Receiver<()>,
    agent_local_device: Receiver<()>,
}

pub fn run() -> AppResult<()> {
    use helpers::create_ctrl_channel;
    let app_config = AppConfig::parse();
    // println!("{:?}", app_config);
    // TODO: Implement clap

    let ctrl_c_events = create_ctrl_channel()?;
    // comm::print_ports();

    // let port_path = "/dev/ttyS11";
    // let baud_rate = "9600";
    let maybe_port_path = app_config.port_path.and_then(|portpath| {
        portpath
            .to_str()
            .and_then(|port_str| Some(port_str.to_string()))
    });
    let maybe_baud_rate = &app_config.baud_rate;
    if let None = maybe_port_path {
        return Err(anyhow::anyhow!("Invalid port_path"));
    }
    if let None = maybe_baud_rate {
        return Err(anyhow::anyhow!("Invalid baud_rate"));
    }
    if let Some(baud_rate) = maybe_baud_rate {
        // baud_rate is the desired baud rate setting for the RS-232 serial
        // console port in bits per second (bps).
        // Valid values are 110, 300, 1200, 2400, 4800, 9600, 19200,
        // 38400, 57600, or 115200.
        if baud_rate > &115_200 {
            return Err(anyhow::anyhow!(
                "Invalid baud_rate value: [{baud_rate}]. Try 9600"
            ));
        }
    }
    // TODO: Some bare tests needed
    if let (Some(port_path), Some(baud_rate)) = (maybe_port_path, app_config.baud_rate) {
        serialport_listener::run(&port_path, baud_rate, ctrl_c_events)?;
    } else {
        return Err(anyhow::anyhow!(
            "Impossible state on getting arguments port_path and baud_rate"
        ));
    }

    // comm::open_port("/dev/ttyS11")?;

    // comm::open_port("/dev/ttyS12")?;
    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
