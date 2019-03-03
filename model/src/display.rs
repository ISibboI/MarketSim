use std::fmt::Display;
use crate::world::World;
use crate::entity::Entity;
use std::fmt::Formatter;
use std::fmt::Error;
use crate::market::Market;
use crate::market::offer::Offer;

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
        write!(f, "; Wares n (p):")?;
        for ware_type in self.wares().iter() {
            write!(f, "{:?} {} ({})", ware_type, self.wares().ware_amount(ware_type), self.prices().get_price(ware_type))?;
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
        write!(f, "{}x {} -> {}x {}", self.offer().amount(), self.offer().ware_type(), self)
    }
}