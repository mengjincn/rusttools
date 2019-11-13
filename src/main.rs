use structopt::StructOpt;
use indicatif::ProgressBar;
use std::time::Duration;
use log::{info, warn, debug, error};
mod tool;
fn main() {
    env_logger::init();

    let cli:Cli = Cli::from_args();

    if cli.timestamp!=-1 {
        println!("{}: {}", cli.timestamp, tool::convert_timestamp_to_date(cli.timestamp));
    }
    if cli.encode.is_some() {
        let string = &(cli.encode.unwrap());
        println!("encode {} : {}", string, tool::encode_to_base64(string));
    }
    if cli.decode.is_some() {
        let string = &(cli.decode.unwrap());
        println!("decode {} : {}", string, tool::decode_of_base64(string));
    }
//
//    debug!("debug info");
//    info!("starting up");
//    warn!("oops, nothing implemented!");
//    let pb = indicatif::ProgressBar::new(100);
//    for i in 0..100 {
//        std::thread::sleep(Duration::from_millis(10));
//        if i % 10 == 0 {
//            pb.println(format!("[+] finished #{}\t", i));
//        }
//        pb.inc(1);
//    }
//    pb.finish_with_message("done");
}


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
#[structopt(name = "tools", about = "A collection tools of daily use.")]
struct Cli {
    #[structopt(short = "t",required=false,default_value="-1")]
    timestamp:i64,

    /// The string will be encode to base64
    #[structopt(long)]
    encode: Option<String>,
    /// The base64 string will be decode
    #[structopt(long)]
    decode: Option<String>,
}