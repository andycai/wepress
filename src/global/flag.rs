use clap::Parser;
use lazy_static::lazy_static;

/// Wepress is a lightweight headless CMS written in Rust.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Address of `TcpListener` bind
    #[arg(short, long, default_value = "")]
    pub addr: String,

    /// Datebase driver
    #[arg(long, default_value = "sqlite")]
    pub db_driver: String,

    /// Database DNS
    #[arg(short, long, default_value = "wepress.db")]
    pub dsn: String,

    /// Debug mode
    #[arg(long, default_value = "false")]
    pub debug: bool,

    /// Log file
    #[arg(short, long, default_value = "wepress.log")]
    pub log: String,
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}
