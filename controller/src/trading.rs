use model::{
    entity::Entity,
    market::{offer::OfferType, Market},
    ware::{WareAmount, WareStore, WareType},
    world::World,
};
use rand::seq::SliceRandom;
use std::cmp::Ordering;

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

                let price_per_ware = entity.sell_prices().single_price_as_ware(&tradable_ware);
                entity.add_offer_id(market.create_offer(
                    tradable_ware,
                    OfferType::Sell,
                    price_per_ware,
                    entity_id,
                ));
            }

            let mut money = tradable_wares.ware_amount(WareType::Money);
            let mut unmet_demands: Vec<_> = unmet_demands.iter().collect();
            unmet_demands.shuffle(&mut rand::thread_rng());

            for unmet_demand in unmet_demands.iter() {
                if unmet_demand.is_money() {
                    continue;
                }

                let price_per_ware = entity.buy_prices().single_price_as_ware(&unmet_demand);
                let max_buy = money / price_per_ware.amount();
                let mut unmet_demand = unmet_demand.clone();
                *unmet_demand.amount_mut() = unmet_demand.amount().min(max_buy);
                if unmet_demand.amount() > 0 {
                    money -= unmet_demand.amount() * price_per_ware.amount();
                    entity.add_offer_id(market.create_offer(
                        unmet_demand,
                        OfferType::Buy,
                        price_per_ware,
                        entity_id,
                    ));
                }
            }
        }

        market.sort_offers();
    }
}

pub trait RandomizedMarket {
    fn resolve_trades(&mut self);
}

impl RandomizedMarket for Market {
    fn resolve_trades(&mut self) {
        println!("{}", self);

        for ware_range in self.iter_ware_ranges_mut() {
            // Iterate over minimum price of sell offers.
            let sell_offer_limits = ware_range.sell_offer_limits();

            for sell_offer_limit in sell_offer_limits {
                let sell_offer_size: WareAmount = sell_offer_limit
                    .sell_slice(ware_range.sell_offers())
                    .iter()
                    .map(|offer| offer.amount())
                    .sum();
                let buy_offer_size: WareAmount = sell_offer_limit
                    .buy_slice(ware_range.buy_offers())
                    .iter()
                    .map(|offer| offer.amount())
                    .sum();

                match sell_offer_size.cmp(&buy_offer_size) {
                    Ordering::Equal => {
                        // Fulfill all offers at minimum price.
                    }
                    _ => unimplemented!(),
                }
            }

            // Fulfill the offers fairly at random, using GSL::multinomial() to deal out the minority to the majority.

            // Offers are fulfilled at minimum price.
        }

        unimplemented!()
    }
}

impl RandomizedMarket for World {
    fn resolve_trades(&mut self) {
        self.market_mut().resolve_trades()
    }
}

#[cfg(test)]
mod test {
    use crate::trading::{Economy, RandomizedMarket, Trader};
    use model::{
        entity::{recipe::Recipe, Entity},
        market::{offer::OfferType, Market},
        templates::{EATING_RECIPE, FOOD_CREATOR_RECIPE},
        ware::{Ware, WareStore, WareType},
        world::World,
    };
    use rand::{distributions::Uniform, Rng, SeedableRng};
    use rand_pcg::Pcg64Mcg;
    use std::str::FromStr;

    #[test]
    fn test_tradable_wares_and_unmet_demands() {
        let mut entity = Entity::new(
            "Bob".to_owned(),
            vec![Recipe::new(vec![], vec![Ware::new(WareType::Food, 1)])],
        );
        entity.add_ware(Ware::new(WareType::Food, 10));

        let mut tradable_wares = WareStore::new();
        tradable_wares.push_ware(Ware::new(WareType::Food, 10));
        let unmet_demands = WareStore::new();

        assert_eq!(
            (tradable_wares, unmet_demands),
            entity.tradable_wares_and_unmet_demands()
        );
    }

    #[test]
    fn test_update_market_offers() {
        let mut world = World::new();
        world.create_entity(
            "Alice",
            &[Recipe::new(vec![Ware::new(WareType::Food, 1)], vec![])],
        );
        world.create_entity(
            "Bob",
            &[Recipe::new(vec![], vec![Ware::new(WareType::Food, 1)])],
        );
        world
            .get_entity_mut(0)
            .add_ware(Ware::new(WareType::Money, 50));
        world
            .get_entity_mut(1)
            .add_ware(Ware::new(WareType::Food, 10));
        world.update_market_offers();

        let mut market = Market::new();
        market.create_offer(
            Ware::new(WareType::Food, 1),
            OfferType::Buy,
            Ware::new(WareType::Money, 5),
            0,
        );
        market.create_offer(
            Ware::new(WareType::Food, 10),
            OfferType::Sell,
            Ware::new(WareType::Money, 5),
            1,
        );

        assert_eq!(&market, world.market());
    }

    #[test]
    fn test_update_market_offers_no_redundant_offers() {
        let mut world = World::new();
        let eating_recipe = Recipe::from_str(EATING_RECIPE).unwrap();
        world.create_entity(
            "Human",
            &[
                eating_recipe.clone(),
                eating_recipe.clone(),
                eating_recipe.clone(),
            ],
        );
        world.get_entity_mut(0).add_ware(Ware::money(100));
        world.update_market_offers();
        assert_eq!(world.market().offers().len(), 1);
    }

    #[test]
    fn test_resolve_trades() {
        let mut rng: Pcg64Mcg = SeedableRng::from_seed([0; 16]);
        let food_price_distribution = Uniform::new_inclusive(4, 6);
        let mut world = World::new();
        let humans: Vec<_> = (0..10)
            .map(|i| {
                world.create_entity(
                    &format!("Human {}", i),
                    &[Recipe::from_str(EATING_RECIPE).unwrap()],
                )
            })
            .collect();
        let food_creators: Vec<_> = (0..10)
            .map(|i| {
                world.create_entity(
                    &format!("Food Creator {}", i),
                    &[Recipe::from_str(FOOD_CREATOR_RECIPE).unwrap()],
                )
            })
            .collect();

        for &human_id in &humans {
            world
                .get_entity_mut(human_id)
                .add_ware(Ware::new(WareType::Money, 50));
        }

        for &food_creator_id in &food_creators {
            world
                .get_entity_mut(food_creator_id)
                .add_ware(Ware::new(WareType::Food, 10));
        }

        for entity in world.entities_mut() {
            entity
                .buy_prices_mut()
                .set_single_price(WareType::Food, rng.sample(food_price_distribution) + 1);
            entity
                .sell_prices_mut()
                .set_single_price(WareType::Food, rng.sample(food_price_distribution));
        }

        world.update_market_offers();
        world.market_mut().resolve_trades();

        unimplemented!()
    }
}
