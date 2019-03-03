use crate::market::offer::Offer;

pub mod offer;

#[derive(Clone, Debug, Default)]
pub struct Market {
    offers: Vec<Offer>,
}

impl Market {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn offers(&self) -> &[Offer] {
        &self.offers
    }

    pub fn offers_mut(&mut self) -> &mut [Offer] {
        &mut self.offers
    }
}