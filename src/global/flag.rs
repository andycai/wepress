use clap::Parser;
use lazy_static::lazy_static;

/// Wepress is a lightweight headless CMS written in Rust.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Address of `TcpListener` bind
    #[arg(short, long, default_value = "0.0.0.0:3000")]
    addr: String,

    /// Datebase driver
    #[arg(short, long, default_value = "sqlite")]
    db_driver: String,

    /// Database DNS
    #[arg(short, long, default_value = "wepress.db")]
    dsn: String,

    /// Debug mode
    #[arg(short, long)]
    debug: bool,

    /// Log file
    #[arg(short, long, default_value = "wepress.log")]
    log: String,
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}
