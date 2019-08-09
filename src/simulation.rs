use crate::frontend::ApiResponse;
use crate::simulation::simulation_state::{SimulationState, SimulationStateHandle, BuyerHandle};
use ordslice::Ext;
use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rand::distributions::Uniform;

pub mod simulation_state;

pub struct Simulator {
    simulation_state: SimulationState,
    history: ApiResponse,
    random: StdRng,
}

impl From<SimulationState> for Simulator {
    fn from(simulation_state: SimulationState) -> Self {
        Simulator {simulation_state, history: ApiResponse::default(), random: StdRng::from_entropy()}
    }
}

impl Simulator {
    pub fn update(&mut self) -> ApiResponse {
        let mut buyers = self.simulation_state.get_sorted_buyers();
        let sellers = self.simulation_state.get_sorted_sellers();

        for seller in &sellers {
            let price = *seller.get(&self.simulation_state).expected_price();
            let min_buyer = buyers.lower_bound_by(|a: &BuyerHandle| {
                a.get(&self.simulation_state).expected_price().cmp(&price)
            });
            if min_buyer < buyers.len() {
                let buyer_distribution = Uniform::new(min_buyer, buyers.len());
                let buyer = self.random.sample(buyer_distribution);
                let buyer = buyers.remove(buyer);
                buyer.get_mut(&mut self.simulation_state).decrease_expected_price();
                seller.get_mut(&mut self.simulation_state).increase_expected_price();
            } else {
                seller.get_mut(&mut self.simulation_state).decrease_expected_price();
            }
        }

        for buyer in &buyers {
            buyer.get_mut(&mut self.simulation_state).increase_expected_price();
        }
        let average_price = sellers.iter().map(|seller| *seller.get(&self.simulation_state).expected_price() as f32).sum::<f32>() / sellers.len() as f32;

        self.history.add_point(average_price);
        self.history.clone()
    }
}