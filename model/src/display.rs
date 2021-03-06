use crate::{
    entity::{recipe::Recipe, Entity},
    market::{
        offer::{Offer, OfferType},
        Market,
    },
    ware::{Ware, WareStore, WareType},
    world::World,
};
use std::fmt::{Debug, Display, Error, Formatter};

impl Display for World {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "World")?;
        for entity in self.entities() {
            write!(f, "\n{}", entity)?;
        }
        write!(f, "\n{}", self.market())
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:.12}: Recipes:", self.name())?;
        for recipe in self.recipes() {
            write!(f, " {}", recipe)?;
        }
        write!(f, "; ||| Wares n (b/s):")?;
        for ware_type in self.wares().iter_ware_types() {
            write!(
                f,
                " {:?} {} ({}/{})",
                ware_type,
                self.wares().ware_amount(ware_type),
                self.buy_prices().single_price(ware_type),
                self.sell_prices().single_price(ware_type)
            )?;
        }
        Ok(())
    }
}

impl Display for Market {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Market")?;
        for offer in self.offers() {
            write!(f, "\n{}", offer)?;
        }
        Ok(())
    }
}

impl Display for Offer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.offer_type() {
            OfferType::Buy => write!(f, "BUY ")?,
            OfferType::Sell => write!(f, "SELL ")?,
        };
        write!(f, "{} at {}/unit", self.offer(), self.price_per_ware())
    }
}

impl Display for WareType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Debug::fmt(self, f)
    }
}

impl Display for Recipe {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "(")?;
        let mut once = false;
        for input in self.inputs() {
            if once {
                write!(f, "; ")?;
            } else {
                once = true;
            }
            write!(f, "{}", input)?;
        }
        write!(f, ") -> (")?;
        let mut once = false;
        for output in self.outputs() {
            if once {
                write!(f, "; ")?;
            } else {
                once = true;
            }
            write!(f, "{}", output)?;
        }
        write!(f, ")")
    }
}

impl Display for Ware {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}x {}", self.amount(), self.ware_type())
    }
}

impl Display for WareStore {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut once = false;
        for ware_type in self.iter_ware_types() {
            if once {
                write!(f, "; ")?;
            } else {
                once = true;
            }
            write!(f, "{}x {}", self.ware_amount(ware_type), ware_type)?;
        }
        Ok(())
    }
}
