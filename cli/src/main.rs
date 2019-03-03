extern crate core;
#[macro_use]
extern crate log;
extern crate simplelog;

use core::world::World;
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, TermLogger, WriteLogger};
use std::fs::File;
use core::entity::recipe::Recipe;
use core::ware::Ware;
use core::ware::WareType;

fn init_loggers() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Trace, Config::default()).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("cli.log").unwrap(),
        ),
    ])
    .unwrap();

    info!("Loggers initialized");
}

fn main() {
    println!("Hello!");
    init_loggers();

    let mut world = World::new();
    world.create_entity("Human", &[Recipe::new(vec![Ware::new(WareType::Food, 1)], Vec::new())]);

    info!("\n{:?}", world);

    println!("Goodbye!");
}
