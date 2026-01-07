use anyhow::Result;
use tun_helper::{disable_tun, enable_tun};

use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct StartArgs {
    /// Tun program path
    #[arg(long)]
    program_path: String,

    /// Tun configuration path
    #[arg(long)]
    config_path: String,

    /// Tun gateway
    #[arg(long)]
    gateway: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct StopArgs {
    /// Tun pid file path
    #[arg(long)]
    pid_file: String,
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
    let args = TunHelperCli::parse();

    println!("args: {:?}", args);

    match args {
        TunHelperCli::Start(args) => {
            enable_tun(&args.program_path, &args.config_path, &args.gateway).await?;
        }
        TunHelperCli::Stop(args) => {
            disable_tun(args.pid_file)?;
        }
    }

    Ok(())
}
