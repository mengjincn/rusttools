use structopt::StructOpt;
use indicatif::ProgressBar;
use std::time::Duration;
use log::{info, warn, debug, error};
mod tool;
fn main() {
    env_logger::init();

    let cli:Cli = Cli::from_args();

    if cli.timestamp>0 {
        println!("{}: {}", cli.timestamp, tool::convert_timestamp_to_date(cli.timestamp));
    }
    println!("cli: {:?}", cli);
    debug!("debug info");
    info!("starting up");
    warn!("oops, nothing implemented!");
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        std::thread::sleep(Duration::from_millis(10));
        if i % 10 == 0 {
            pb.println(format!("[+] finished #{}\t", i));
        }
        pb.inc(1);
    }
    pb.finish_with_message("done");
}


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
#[structopt(name = "tools", about = "A collection tools of daily use.")]
struct Cli {
    #[structopt(short = "t")]
    timestamp:i64,

    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}