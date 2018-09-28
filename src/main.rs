extern crate byte_unit;
#[macro_use]
extern crate clap;

use byte_unit::Byte;
use clap::Arg;
use std::thread;
use std::time::Duration;

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name("RESERVE_SIZE").index(1).required(true))
        .get_matches();

    let size = matches.value_of("RESERVE_SIZE").unwrap();
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
