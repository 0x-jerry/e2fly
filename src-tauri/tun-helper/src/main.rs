use anyhow::{Result, bail};
use std::env;
use tun_helper::{disable_tun, enable_tun};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct StartArgs {
    /// Tun program path
    #[arg(long)]
    program_path: String,

    /// Tun configuration path
    #[arg(long)]
    config_path: String,

    /// Tun pid file path
    #[arg(long)]
    pid_path: String,

    /// Tun interface name
    #[arg(long)]
    interface_name: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct StopArgs {
    /// Tun pid file path
    #[arg(long)]
    pid_path: String,
}

#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "tun-helper")]
#[command(bin_name = "tun-helper")]
enum TunHelperCli {
    /// Start tun
    Start(StartArgs),
    /// Stop tun
    Stop(StopArgs),
}

#[tokio::main]
pub async fn main() -> Result<()> {
    init_logs()?;

    log::info!("Start parse args: {:?}", env::args());

    let args = TunHelperCli::parse();

    log::info!("Parsed args: {:?}", args);

    match args {
        TunHelperCli::Start(args) => {
            if let Err(err) = enable_tun(
                &args.program_path,
                &args.config_path,
                &args.pid_path,
                &args.interface_name,
            )
            .await
            {
                log::error!("{}", err)
            };
        }
        TunHelperCli::Stop(args) => {
            if let Err(err) = disable_tun(args.pid_path) {
                log::error!("{}", err);
            };
        }
    }

    Ok(())
}

fn init_logs() -> Result<()> {
    use log::LevelFilter;
    use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
    use std::fs::File;

    let log_file_path = match env::current_exe() {
        Ok(exe) => {
            log::info!("exe path: {:?}", exe);

            if exe.parent().is_none() {
                bail!("Can not find exe dir");
            }

            exe.parent().unwrap().join("tun-helper.log")
        }
        Err(_) => {
            log::error!("Failed to get current executable path");
            bail!("initlized logs failed!");
        }
    };

    let file = File::create(&log_file_path)?;

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, Config::default(), file),
    ])?;

    Ok(())
}
