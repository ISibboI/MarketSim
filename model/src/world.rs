use crate::{
    entity::{recipe::Recipe, Entity},
    market::Market,
};
use crate::ware::Ware;
use crate::market::OfferId;
use crate::market::offer::OfferType;

pub type EntityId = usize;

#[derive(Clone, Debug, Default)]
pub struct World {
    entities: Vec<Entity>,
    market: Market,
}

// Creators
impl World {
    pub fn new() -> Self {
        trace!("Created new world");

        Default::default()
    }

    pub fn create_entity(&mut self, name: &str, recipes: &[Recipe]) -> EntityId {
        let entity = Entity::new(name.to_owned(), recipes.to_owned());
        let entity_id = self.entities.len() as EntityId;
        trace!("Creating entity {:?} with id {:?}", entity, entity_id);

        self.entities.push(entity);
        entity_id
    }

    pub fn create_offer(&mut self, entity_id: EntityId, offer: Ware, offer_type: OfferType, price_per_ware: Ware) -> Result<OfferId, ()> {
        let offer = self.entities_mut()[entity_id as usize].remove_ware(offer)?;
        Ok(self.market_mut().create_offer(offer, offer_type, price_per_ware, entity_id))
    }
}

// Getters
impl World {
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn market(&self) -> &Market {
        &self.market
    }

    pub fn entities_mut(&mut self) -> &mut [Entity] {
        &mut self.entities
    }

    pub fn market_mut(&mut self) -> &mut Market {
        &mut self.market
    }

    pub fn entities_market_mut(&mut self) -> (&mut [Entity], &mut Market) {
        (&mut self.entities, &mut self.market)
    }

    pub fn get_entity(&self, entity_id: EntityId) -> &Entity {
        &self.entities[entity_id as usize]
    }

    pub fn get_entity_mut(&mut self, entity_id: EntityId) -> &mut Entity {
        &mut self.entities[entity_id as usize]
    }
}
