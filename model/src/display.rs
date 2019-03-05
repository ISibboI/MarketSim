use std::fmt::Display;
use crate::world::World;
use crate::entity::Entity;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Debug;
use crate::market::Market;
use crate::market::offer::Offer;
use crate::ware::{WareType, Ware};
use crate::entity::recipe::Recipe;

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
        write!(f, "; ||| Wares n (p):")?;
        for ware_type in self.wares().iter() {
            write!(f, " {:?} {} ({})", ware_type, self.wares().ware_amount(ware_type), self.prices().get_price(ware_type))?;
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
        write!(f, "{} -> {}", self.offer(), self.price())
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