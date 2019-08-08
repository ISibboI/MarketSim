#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate getset;

mod frontend;
mod simulation;

use crate::simulation::simulation_state::{SimulationState};
use crate::simulation::simulation_state::entities::{BuyerProperties, SellerProperties};


fn main() {
    let mut initial_simulation_state = SimulationState::default();
    initial_simulation_state.add_buyer(10.into());
    initial_simulation_state.add_seller(5.into());

    frontend::run(initial_simulation_state);
}
