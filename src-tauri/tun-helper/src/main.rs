use anyhow::Result;
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
    {
        use log::LevelFilter;
        use simplelog::{
            ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger,
        };
        use std::fs::File;
        CombinedLogger::init(vec![
            TermLogger::new(
                LevelFilter::Debug,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                File::create("tun-helper.log").unwrap(),
            ),
        ])
        .unwrap();
    }

    let args = TunHelperCli::parse();

    log::info!("args: {:?}", args);

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
