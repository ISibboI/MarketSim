use crate::market::offer::Offer;
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

    pub fn create_offer(&mut self, offer: Ware, price: Ware, entity_id: EntityId) -> OfferId {
        let offer_id = self.offers().len() as OfferId;
        self.offers_mut().push(Offer::new(offer, price, entity_id));
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
