use crate::{
    market::{
        offer::{Offer, OfferType},
        Market,
    },
    ware::WareAmount,
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

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(first_offer) = self.offers.first().cloned() {
            let count = self
                .offers
                .iter()
                .skip(1)
                .take_while(|offer| {
                    offer.offer().ware_type() == first_offer.offer().ware_type()
                        && offer.offer_type() == OfferType::Buy
                })
                .count()
                + 1;
            let tmp = mem::replace(&mut self.offers, &mut []);
            let (buy_offers, new_offers) = tmp.split_at_mut(count);
            self.offers = new_offers;

            let count = self
                .offers
                .iter()
                .take_while(|offer| {
                    offer.offer().ware_type() == first_offer.offer().ware_type()
                        && offer.offer_type() == OfferType::Sell
                })
                .count()
                + 1;
            let tmp = mem::replace(&mut self.offers, &mut []);
            let (sell_offers, new_offers) = tmp.split_at_mut(count);
            self.offers = new_offers;

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

    pub fn sell_offer_limits(&self) -> Vec<SellOfferLimit> {
        let mut result = Vec::new();
        if let Some(first) = self.sell_offers().first() {
            let mut price = first.price_per_ware().amount();

            for (sell_offer_id, sell_offer) in self.sell_offers().iter().enumerate() {
                if sell_offer.price_per_ware().amount() != price {
                    let buy_offset = result
                        .last()
                        .unwrap_or(&SellOfferLimit::zero())
                        .buy_offset();
                    result.push(SellOfferLimit::new(
                        price,
                        result
                            .last()
                            .unwrap_or(&SellOfferLimit::zero())
                            .sell_limit(),
                        sell_offer_id,
                        self.buy_offers()
                            .iter()
                            .enumerate()
                            .skip(buy_offset)
                            .skip_while(|(_, offer)| offer.price_per_ware().amount() < price)
                            .map(|(i, _)| i)
                            .next()
                            .unwrap_or(self.buy_offers().len()),
                        self.buy_offers().len(),
                    ));
                    price = sell_offer.price_per_ware().amount();
                }
            }
        }
        result
    }
}

pub struct SellOfferLimit {
    price: WareAmount,
    sell_offset: usize,
    sell_limit: usize,
    buy_offset: usize,
    buy_limit: usize,
}

impl SellOfferLimit {
    fn new(
        price: WareAmount,
        sell_offset: usize,
        sell_limit: usize,
        buy_offset: usize,
        buy_limit: usize,
    ) -> Self {
        Self {
            price,
            sell_offset,
            sell_limit,
            buy_offset,
            buy_limit,
        }
    }

    fn zero() -> Self {
        Self::new(0, 0, 0, 0, 0)
    }

    pub fn price(&self) -> WareAmount {
        self.price
    }

    pub fn sell_offset(&self) -> usize {
        self.sell_offset
    }

    pub fn sell_limit(&self) -> usize {
        self.sell_limit
    }

    pub fn buy_offset(&self) -> usize {
        self.buy_offset
    }

    pub fn buy_limit(&self) -> usize {
        self.buy_limit
    }

    pub fn sell_slice<'a>(&self, sell_offers: &'a [Offer]) -> &'a [Offer] {
        &sell_offers[self.sell_offset()..self.sell_limit()]
    }

    pub fn buy_slice<'a>(&self, buy_offers: &'a [Offer]) -> &'a [Offer] {
        &buy_offers[self.buy_offset()..self.buy_limit()]
    }

    pub fn sell_slice_mut<'a>(&self, sell_offers: &'a mut [Offer]) -> &'a mut [Offer] {
        &mut sell_offers[self.sell_offset()..self.sell_limit()]
    }

    pub fn buy_slice_mut<'a>(&self, buy_offers: &'a mut [Offer]) -> &'a mut [Offer] {
        &mut buy_offers[self.buy_offset()..self.buy_limit()]
    }
}
