use model::entity::Entity;
use model::world::World;
use model::ware::{WareStore, WareType};

pub trait Trader {
    fn tradable_wares_and_unmet_demands(&self) -> (WareStore, WareStore);
}

impl Trader for Entity {
    fn tradable_wares_and_unmet_demands(&self) -> (WareStore, WareStore) {
        let mut demands = WareStore::new();

        for recipe in self.recipes() {
            for input in recipe.inputs() {
                demands.push_ware(input.clone());
            }
        }

        let mut tradable_wares = self.wares().clone();
        tradable_wares.pop_wares_max(&mut demands);
        (tradable_wares, demands)
    }
}

pub trait Economy {
    fn update_market_offers(&mut self);
}

impl Economy for World {
    fn update_market_offers(&mut self) {
        let (entities, market) = self.entities_market_mut();

        market.clear_offers();

        for (entity_id, entity) in entities.iter_mut().enumerate() {
            let (tradable_wares, unmet_demands) = entity.tradable_wares_and_unmet_demands();
            entity.clear_offer_ids();

            for tradable_ware in tradable_wares.iter() {
                if tradable_ware.is_money() {
                    continue;
                }

                let price = entity.prices().price(&tradable_ware);
                entity.add_offer_id(market.create_offer(tradable_ware, price, entity_id));
            }

            let mut money = tradable_wares.ware_amount(WareType::Money);
            for unmet_demand in unmet_demands.iter() {
                if unmet_demand.is_money() {
                    continue;
                }


            }
        }
    }
}

#[cfg(test)]
mod test {
    use model::world::World;
    use model::entity::recipe::Recipe;
    use model::ware::{Ware, WareType, WareStore};
    use crate::trading::{Economy, Trader};
    use model::market::Market;
    use model::entity::Entity;

    #[test]
    fn test_tradable_wares_and_unmet_demands() {
        let mut entity = Entity::new("Bob".to_owned(), vec![Recipe::new(vec![], vec![Ware::new(WareType::Food, 1)])]);
        entity.add_ware(Ware::new(WareType::Food, 10));

        let mut tradable_wares = WareStore::new();
        tradable_wares.push_ware(Ware::new(WareType::Food, 10));
        let unmet_demands = WareStore::new();

        assert_eq!((tradable_wares, unmet_demands), entity.tradable_wares_and_unmet_demands());
    }

    #[test]
    fn test_update_market_offers() {
        let mut world = World::new();
        world.create_entity("Alice", &[Recipe::new(vec![Ware::new(WareType::Food, 1)], vec![])]);
        world.create_entity("Bob", &[Recipe::new(vec![], vec![Ware::new(WareType::Food, 1)])]);
        world.get_entity_mut(0).add_ware(Ware::new(WareType::Money, 50));
        world.get_entity_mut(1).add_ware(Ware::new(WareType::Food, 10));
        world.update_market_offers();

        let mut market = Market::new();
        market.create_offer(Ware::new(WareType::Money, 5), Ware::new(WareType::Food, 1), 0);
        market.create_offer(Ware::new(WareType::Food, 10), Ware::new(WareType::Money, 50), 1);

        assert_eq!(&market, world.market());
    }
}