use std::ffi::OsString;

use clap::Parser;

mod capture;
mod capture_file;
mod capture_header;
mod endianness_aware_cursor;
mod error;
mod five_tuple;
mod packet;
mod packet_dissection;
mod packet_header;
mod packet_layer;
mod ui;

fn main() {
    let args = Args::parse();

    ui::run(args.file_path).unwrap();
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = None)]
    file_path: Option<OsString>,
}
