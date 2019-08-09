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
extern crate ordslice;
extern crate rand;

mod frontend;
mod simulation;

use crate::simulation::simulation_state::{SimulationState};
use rand::distributions::Uniform;
use rand::Rng;


fn main() {
    let mut initial_simulation_state = SimulationState::default();
    let mut rng = rand::thread_rng();
    let buyer_distribution = Uniform::new_inclusive(10, 100);
    let seller_distribution = Uniform::new_inclusive(5, 50);

    for _ in 0..100 {initial_simulation_state.add_buyer(rng.sample(buyer_distribution).into());}
    for _ in 0..100 {initial_simulation_state.add_seller(rng.sample(seller_distribution).into());}
    initial_simulation_state.add_buyer(11.into());

    frontend::run(initial_simulation_state);
}
