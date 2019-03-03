extern crate core;
#[macro_use]
extern crate log;
extern crate simplelog;

use core::world::World;
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, TermLogger, WriteLogger};
use std::fs::File;

fn init_loggers() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Trace, Config::default()).unwrap(),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("cli.log").unwrap(),
        ),
    ])
    .unwrap();
}

fn main() {
    println!("Hello!");
    init_loggers();

    let world = World::new();
    println!("{:?}", world);

    println!("Goodbye!");
}
