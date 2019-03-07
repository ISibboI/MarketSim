#![feature(is_sorted)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
extern crate failure;

pub mod display;
pub mod entity;
pub mod market;
pub mod prices;
pub mod ware;
pub mod world;
