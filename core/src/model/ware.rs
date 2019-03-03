#[derive(Clone, Debug, Copy)]
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
