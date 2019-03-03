use std::collections::HashMap;

#[derive(Clone, Debug, Copy, Hash, PartialEq, Eq)]
pub enum WareType {
    Food,
    Water,
    Soil,
    Money,
}

impl Default for WareType {
    fn default() -> Self {
        WareType::Money
    }
}

impl WareType {
    pub fn default_price(&self) -> WareAmount {
        use WareType::*;
        match self {
            Food => 5,
            Water => 1,
            Soil => 1,
            Money => 1,
        }
    }

    pub fn is_money(&self) -> bool {
        *self == WareType::Money
    }
}

pub type WareAmount = u32;

#[derive(Default, Clone, Debug)]
pub struct Ware {
    ware_type: WareType,
    amount: WareAmount,
}

impl Ware {
    pub fn new(ware_type: WareType, amount: WareAmount) -> Self {
        Self { ware_type, amount }
    }

    pub fn ware_type(&self) -> WareType {
        self.ware_type
    }

    pub fn amount(&self) -> WareAmount {
        self.amount
    }

    pub fn amount_mut(&mut self) -> &mut WareAmount {
        &mut self.amount
    }
}

#[derive(Debug, Default, Clone)]
pub struct WareStore {
    wares: HashMap<WareType, WareAmount>,
}

// Creators
impl WareStore {
    pub fn new() -> Self {
        Default::default()
    }
}

// Modifiers
impl WareStore {
    pub fn push_ware(&mut self, ware: Ware) {
        trace!("Pushing {:?}", ware);

        if let Some(amount) = self.wares.get_mut(&ware.ware_type()) {
            *amount += ware.amount();
        } else {
            self.wares.insert(ware.ware_type(), ware.amount());
        }
    }

    pub fn pop_ware(&mut self, ware: Ware) -> Result<Ware, ()> {
        if ware.amount() == 0 {
            return Ok(ware);
        }

        if let Some(amount) = self.wares.get_mut(&ware.ware_type()) {
            if *amount >= ware.amount() {
                Ok(ware)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn pop_ware_max(&mut self, ware: &mut Ware) -> Ware {
        if let Some(amount) = self.wares.get_mut(&ware.ware_type()) {
            let pop_amount = (*amount - ware.amount()).max(0);
            *amount -= pop_amount;
            *ware.amount_mut() -= pop_amount;
            Ware::new(ware.ware_type(), pop_amount)
        } else {
            Ware::new(ware.ware_type(), 0)
        }
    }

    pub fn pop_wares(&mut self, wares: WareStore) -> Result<WareStore, ()> {
        for ware_type in wares.iter() {
            if self.ware_amount(ware_type) < wares.ware_amount(ware_type) {
                return Err(());
            }
        }

        Ok(wares)
    }
}

// Getters
impl WareStore {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = WareType> + 'a {
        self.wares.keys().cloned()
    }

    pub fn ware_amount(&self, ware_type: WareType) -> WareAmount {
        self.wares.get(&ware_type).cloned().unwrap_or(0)
    }
}