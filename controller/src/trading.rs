use model::market::Market;
use model::entity::Entity;
use model::world::World;
use model::ware::WareStore;

pub trait Trader {
    fn create_offers(&mut self, market: &mut Market);
}

impl Trader for Entity {
    fn create_offers(&mut self, _market: &mut Market) {
        let mut demands = WareStore::new();

        for recipe in self.recipes() {
            for input in recipe.inputs() {
                demands.push_ware(input.clone());
            }
        }

        let _leftover_wares = self.wares().clone().pop_wares_max(&mut demands);
    }
}

pub trait Economy {
    fn execute_trade_day(&mut self);
}

impl Economy for World {
    fn execute_trade_day(&mut self) {
        let (entities, market) = self.entities_market_mut();
        for entity in entities {
            entity.create_offers(market);
        }
    }
}