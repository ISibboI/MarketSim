use crate::ware::Ware;
use crate::world::EntityId;

#[derive(Default, Debug, Clone)]
pub struct Offer {
    offer: Ware,
    price: Ware,
    seller_entity_id: EntityId,
}

impl Offer {
    pub fn new(offer: Ware, price: Ware, seller_entity_id: EntityId) -> Self {
        Self {offer, price, seller_entity_id}
    }

    pub fn offer(&self) -> &Ware {
        &self.offer
    }

    pub fn price(&self) -> &Ware {
        &self.price
    }

    pub fn seller_entity_id(&self) -> EntityId {
        self.seller_entity_id
    }
}