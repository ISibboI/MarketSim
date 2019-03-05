use crate::{
    ware::{Ware, WareAmount},
    world::EntityId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offer {
    offer: Ware,
    offer_type: OfferType,
    price_per_ware: Ware,
    seller_entity_id: EntityId,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum OfferType {
    Buy,
    Sell,
}

impl Offer {
    pub fn new(
        offer: Ware,
        offer_type: OfferType,
        price_per_ware: Ware,
        seller_entity_id: EntityId,
    ) -> Self {
        Self {
            offer,
            offer_type,
            price_per_ware,
            seller_entity_id,
        }
    }

    pub fn offer(&self) -> &Ware {
        &self.offer
    }

    pub fn amount(&self) -> WareAmount {
        self.offer.amount()
    }

    pub fn price_per_ware(&self) -> &Ware {
        &self.price_per_ware
    }

    pub fn total_price(&self) -> Ware {
        let mut total_price = self.price_per_ware.clone();
        *total_price.amount_mut() *= self.amount();
        total_price
    }

    pub fn seller_entity_id(&self) -> EntityId {
        self.seller_entity_id
    }
}
