use crate::{entity::recipe::Recipe, ware::Ware};

pub mod recipe;

#[derive(Clone, Debug, Default)]
pub struct Entity {
    name: String,
    wares: Vec<Ware>,
    recipes: Vec<Recipe>,
}

impl Entity {
    pub fn new(name: String, recipes: Vec<Recipe>) -> Self {
        Entity {
            name,
            wares: Default::default(),
            recipes,
        }
    }

    pub fn wares(&self) -> &[Ware] {
        &self.wares
    }

    pub fn wares_mut(&mut self) -> &mut [Ware] {
        &mut self.wares
    }

    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }
}
