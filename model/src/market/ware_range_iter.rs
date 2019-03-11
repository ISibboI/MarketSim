use crate::market::{
    offer::{Offer, OfferType},
    Market,
};
use std::mem;

#[derive(Default, Clone, Debug)]
pub struct WareRangeIter<'a> {
    index: usize,
    offers: &'a [Offer],
}

impl<'a> WareRangeIter<'a> {
    pub fn new(offers: &'a [Offer]) -> Self {
        Self { index: 0, offers }
    }
}

impl<'a> From<&'a Market> for WareRangeIter<'a> {
    fn from(market: &'a Market) -> Self {
        Self::new(market.offers())
    }
}

impl<'a> Iterator for WareRangeIter<'a> {
    type Item = WareOfferRange<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(first_offer) = self.offers.get(self.index) {
            let offset = self.index;
            self.index += self
                .offers
                .iter()
                .skip(self.index)
                .take_while(|offer| {
                    offer.offer().ware_type() == first_offer.offer().ware_type()
                        && offer.offer_type() == OfferType::Buy
                })
                .count();
            let buy_offers = &self.offers[offset..self.index];
            let offset = self.index;
            self.index += self
                .offers
                .iter()
                .skip(self.index)
                .take_while(|offer| {
                    offer.offer().ware_type() == first_offer.offer().ware_type()
                        && offer.offer_type() == OfferType::Sell
                })
                .count();
            let sell_offers = &self.offers[offset..self.index];
            Some(WareOfferRange::new(buy_offers, sell_offers))
        } else {
            None
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct WareOfferRange<'a> {
    buy_offers: &'a [Offer],
    sell_offers: &'a [Offer],
}

impl<'a> WareOfferRange<'a> {
    fn new(buy_offers: &'a [Offer], sell_offers: &'a [Offer]) -> Self {
        Self {
            buy_offers,
            sell_offers,
        }
    }

    pub fn buy_offers(&self) -> &'a [Offer] {
        self.buy_offers
    }

    pub fn sell_offers(&self) -> &'a [Offer] {
        self.sell_offers
    }
}

#[derive(Default, Debug)]
pub struct WareRangeIterMut<'a> {
    offers: &'a mut [Offer],
}

impl<'a> WareRangeIterMut<'a> {
    pub fn new(offers: &'a mut [Offer]) -> Self {
        Self { offers }
    }
}

impl<'a> From<&'a mut Market> for WareRangeIterMut<'a> {
    fn from(market: &'a mut Market) -> Self {
        Self::new(market.offers_mut())
    }
}

impl<'a> Iterator for WareRangeIterMut<'a> {
    type Item = WareOfferRangeMut<'a>;

    fn next<'s>(&'s mut self) -> Option<Self::Item> {
        if let Some(first_offer) = self.offers.first().cloned() {
            let mut dummy = Vec::new();
            let mut dummy_slice_mut = dummy.as_mut_slice();
            mem::swap(&mut dummy_slice_mut, &mut self.offers);

            let count = self
                .offers
                .iter()
                .skip(1)
                .take_while(|offer| {
                    offer.offer().ware_type() == first_offer.offer().ware_type()
                        && offer.offer_type() == OfferType::Buy
                })
                .count();
            let (buy_offers, new_offers) = dummy_slice_mut.split_at_mut(count);
            dummy_slice_mut = new_offers;

            let count = self
                .offers
                .iter()
                .take_while(|offer| {
                    offer.offer().ware_type() == first_offer.offer().ware_type()
                        && offer.offer_type() == OfferType::Sell
                })
                .count();
            let (sell_offers, new_offers) = dummy_slice_mut.split_at_mut(count);
            dummy_slice_mut = new_offers;

            mem::swap(&mut dummy_slice_mut, &mut self.offers);
            Some(WareOfferRangeMut::new(buy_offers, sell_offers))
        } else {
            None
        }
    }
}

#[derive(Default, Debug)]
pub struct WareOfferRangeMut<'a> {
    buy_offers: &'a mut [Offer],
    sell_offers: &'a mut [Offer],
}

impl<'a> WareOfferRangeMut<'a> {
    fn new(buy_offers: &'a mut [Offer], sell_offers: &'a mut [Offer]) -> Self {
        Self {
            buy_offers,
            sell_offers,
        }
    }

    pub fn buy_offers(&'a self) -> &'a [Offer] {
        self.buy_offers
    }

    pub fn sell_offers(&'a self) -> &'a [Offer] {
        self.sell_offers
    }

    pub fn buy_offers_mut(&'a mut self) -> &'a mut [Offer] {
        self.buy_offers
    }

    pub fn sell_offers_mut(&'a mut self) -> &'a mut [Offer] {
        self.sell_offers
    }
}
