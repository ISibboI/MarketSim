use crate::model::{
    entity::{recipe::Recipe, Entity},
    market::Market,
};

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
}
