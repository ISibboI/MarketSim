use crate::market::offer::{Offer, OfferType};
use crate::ware::Ware;
use crate::world::EntityId;

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

    pub fn create_offer(&mut self, offer: Ware, offer_type: OfferType, price_per_ware: Ware, entity_id: EntityId) -> OfferId {
        debug_assert_ne!(offer.amount(), 0);

        let offer_id = self.offers().len() as OfferId;
        self.offers_mut().push(Offer::new(offer, offer_type, price_per_ware, entity_id));
        offer_id
    }
}

// Modifiers
impl Market {
    pub fn clear_offers(&mut self) {
        self.offers.clear();
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
