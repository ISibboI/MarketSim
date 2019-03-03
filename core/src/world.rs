use crate::{entity::Entity, market::Market};

#[derive(Clone, Debug, Default)]
pub struct World {
    entities: Vec<Entity>,
    market: Market,
}

impl World {
    pub fn new() -> Self {
        trace!("Created new world");

        Default::default()
    }

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
}
