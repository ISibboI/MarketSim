use crate::{entity::recipe::Recipe, ware::Ware};
use crate::market::OfferId;
use crate::ware::WareStore;
use crate::prices::PriceTable;

pub mod recipe;

#[derive(Clone, Debug, Default)]
pub struct Entity {
    name: String,
    wares: WareStore,
    prices: PriceTable,
    recipes: Vec<Recipe>,
    offer_ids: Vec<OfferId>,
}

// Creators
impl Entity {
    pub fn new(name: String, recipes: Vec<Recipe>) -> Self {
        Entity {
            name,
            wares: Default::default(),
            prices: Default::default(),
            recipes,
            offer_ids: Default::default(),
        }
    }
}

// Modifiers
impl Entity {
    pub fn remove_ware(&mut self, ware: Ware) -> Result<Ware, ()> {
        self.wares_mut().pop_ware(ware)
    }
    pub fn add_ware(&mut self, ware: Ware) {
        self.wares_mut().push_ware(ware)
    }

    pub fn add_offer_id(&mut self, offer_id: OfferId) {
        self.offer_ids_mut().push(offer_id);
    }

    pub fn clear_offer_ids(&mut self) {
        self.offer_ids.clear();
    }
}

// Getters
impl Entity {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn wares(&self) -> &WareStore {
        &self.wares
    }

    fn wares_mut(&mut self) -> &mut WareStore {
        &mut self.wares
    }

    pub fn prices(&self) -> &PriceTable {
        &self.prices
    }

    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }

    pub fn offer_ids(&self) -> &[OfferId] {
        &self.offer_ids
    }

    fn offer_ids_mut(&mut self) -> &mut Vec<OfferId> {
        &mut self.offer_ids
    }
}
