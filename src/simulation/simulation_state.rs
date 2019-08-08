use crate::simulation::simulation_state::entities::{BuyerProperties, SellerProperties, Buyer, Seller};

pub mod entities;

#[derive(Clone, Copy)]
pub struct BuyerHandle {
    id: usize,
}

#[derive(Clone, Copy)]
pub struct SellerHandle {
    id: usize,
}

pub trait SimulationStateHandle<T> {
    fn get<'a>(&self, simulation_state: &'a SimulationState) -> &'a T;

    fn get_mut<'a>(&self, simulation_state: &'a mut SimulationState) -> &'a mut T;
}

impl SimulationStateHandle<Buyer> for BuyerHandle {
    fn get<'a>(&self, simulation_state: &'a SimulationState) -> &'a Buyer {
        &simulation_state.buyers[self.id]
    }

    fn get_mut<'a>(&self, simulation_state: &'a mut SimulationState) -> &'a mut Buyer {
        &mut simulation_state.buyers[self.id]
    }
}

impl SimulationStateHandle<Seller> for SellerHandle {
    fn get<'a>(&self, simulation_state: &'a SimulationState) -> &'a Seller {
        &simulation_state.sellers[self.id]
    }

    fn get_mut<'a>(&self, simulation_state: &'a mut SimulationState) -> &'a mut Seller {
        &mut simulation_state.sellers[self.id]
    }
}

#[derive(Default, Clone)]
pub struct SimulationState {
    buyers: Vec<Buyer>,
    sellers: Vec<Seller>,
}

impl SimulationState {
    pub fn add_buyer(&mut self, properties: BuyerProperties) -> BuyerHandle {
        let id = self.buyers.len();
        self.buyers.push(Buyer::from(properties));
        BuyerHandle {id}
    }

    pub fn add_seller(&mut self, properties: SellerProperties) -> SellerHandle {
        let id = self.sellers.len();
        self.sellers.push(Seller::from(properties));
        SellerHandle {id}
    }

    pub fn get_sorted_buyers(&self) -> Vec<BuyerHandle> {
        let mut buyers: Vec<_> = (0..self.buyers.len()).map(|id| BuyerHandle {id}).collect();
        buyers.sort_by(|a: &BuyerHandle, b: &BuyerHandle| {
            a.get(self).expected_price().cmp(&b.get(self).expected_price())
        });
        buyers
    }

    pub fn get_sorted_sellers(&self) -> Vec<SellerHandle> {
        let mut sellers: Vec<_> = (0..self.sellers.len()).map(|id| SellerHandle {id}).collect();
        sellers.sort_by(|a: &SellerHandle, b: &SellerHandle| {
            a.get(self).expected_price().cmp(&b.get(self).expected_price())
        });
        sellers
    }
}