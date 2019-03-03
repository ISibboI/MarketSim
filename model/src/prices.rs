use crate::ware::WareAmount;
use crate::ware::WareType;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct PriceTable {
    prices: HashMap<WareType, WareAmount>,
}

// Creators
impl PriceTable {
    pub fn new() -> Self {
        Default::default()
    }
}

// Modifiers
impl PriceTable {
    pub fn set_price(&mut self, ware_type: WareType, price: WareAmount) {
        self.prices_mut().insert(ware_type, price);
    }
}

// Getters
impl PriceTable {
    fn prices(&self) -> &HashMap<WareType, WareAmount> {
        &self.prices
    }

    fn prices_mut(&mut self) -> &mut HashMap<WareType, WareAmount> {
        &mut self.prices
    }

    pub fn get_price(&self, ware_type: WareType) -> WareAmount {
        if let Some(price) = self.prices().get(&ware_type) {
            *price
        } else {
            ware_type.default_price()
        }
    }
}