extern crate byte_unit;
extern crate clap;

use byte_unit::Byte;
use clap::Parser;
use std::thread;
use std::time::Duration;

#[derive(Parser)]
struct Args {
    reserve_size: String,
}

fn main() {
    let args = Args::parse();
    let size = args.reserve_size;
    let size =
        Byte::from_string(&size).unwrap_or_else(|e| panic!("Parse Error: {:?} ({:?})", size, e));

    let _reserve: Vec<u8> = vec![1; size.get_bytes() as usize];
    println!(
        "Reserved Memory Size: {}",
        size.get_appropriate_unit(true).to_string()
    );

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
