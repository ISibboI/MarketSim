use crate::{
    market::offer::{Offer, OfferType},
    ware::Ware,
    world::EntityId,
};
use std::cmp::Ordering;

pub mod offer;

pub type OfferId = usize;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Market {
    offers: Vec<Offer>,
}

// Creators
impl Market {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_offer(
        &mut self,
        offer: Ware,
        offer_type: OfferType,
        price_per_ware: Ware,
        entity_id: EntityId,
    ) -> OfferId {
        debug_assert_ne!(offer.amount(), 0);

        let offer_id = self.offers().len() as OfferId;
        self.offers_mut()
            .push(Offer::new(offer, offer_type, price_per_ware, entity_id));
        offer_id
    }
}

// Modifiers
impl Market {
    pub fn clear_offers(&mut self) {
        self.offers_mut().clear();
    }

    pub fn order_offers(&mut self) {
        self.offers_mut().sort_by(
            |a, b| match a.offer().ware_type().cmp(&b.offer().ware_type()) {
                Ordering::Equal => {
                    if a.offer_type() != b.offer_type() {
                        if a.offer_type() == OfferType::Buy {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    } else {
                        a.price_per_ware()
                            .amount()
                            .cmp(&b.price_per_ware().amount())
                    }
                }
                o => o,
            },
        );
    }
}

// Getters
impl Market {
    pub fn offers(&self) -> &[Offer] {
        &self.offers
    }

    fn offers_mut(&mut self) -> &mut Vec<Offer> {
        &mut self.offers
    }
}

#[cfg(test)]
mod test {
    use crate::{
        market::{offer::OfferType, *},
        ware::{Ware, WareType},
    };
    use rand::{distributions::Uniform, seq::SliceRandom, Rng};

    #[test]
    fn test_order_offers() {
        let mut market = Market::new();

        let possible_ware_types = [WareType::Food, WareType::Water, WareType::Soil];
        let possible_offer_types = [OfferType::Buy, OfferType::Sell];
        let possible_ware_amounts = Uniform::new(1, 16);
        let possible_entity_ids = Uniform::new(0, 22);
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            market.create_offer(
                Ware::new(
                    possible_ware_types.choose(&mut rng).cloned().unwrap(),
                    rng.sample(possible_ware_amounts),
                ),
                possible_offer_types.choose(&mut rng).cloned().unwrap(),
                Ware::new(WareType::Money, rng.sample(possible_ware_amounts)),
                rng.sample(possible_entity_ids),
            );
        }

        assert!(false);
    }
}
