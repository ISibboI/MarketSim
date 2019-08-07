#![feature(proc_macro_hygiene, decl_macro)]

use std::vec::Vec;
use simplelog::{CombinedLogger, TermLogger, WriteLogger, Config};
use log::LevelFilter;
use std::fs::File;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;
extern crate simplelog;

#[derive(Debug, Clone, new)]
struct Buyer {
	pub max_price: i32,
	pub expected_price: i32,
}

#[derive(Debug, Clone, new)]
struct Seller {
	pub min_price: i32,
	pub expected_price: i32,
}

#[get("/")]
fn index() -> &'static str {
	"Hello, world!"
}

fn main() {
	info!("Hello!");

	CombinedLogger::init(
		vec![
			TermLogger::new(LevelFilter::Trace, Config::default()).unwrap(),
			// WriteLogger::new(LevelFilter::Info, Config::default(), File::create("marketsim.log").unwrap()),
		]
	).unwrap();

	let buyers = vec![Buyer::new(10, 5)];
	let sellers = vec![Seller::new(2, 8)];

	rocket::ignite().mount("/", routes![index]).launch();
	info!("Goodbye!");
}
