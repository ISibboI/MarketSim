use crate::{
    entity::recipe::Recipe,
    ware::{Ware, WareAmount, WareType},
};
use std::str::FromStr;

impl FromStr for Ware {
    type Err = String;

    /// Parses a ware from the format '{amount}x {ware_type}'
    ///
    /// # Examples
    ///
    /// ```
    /// use model::ware::*;
    /// use std::str::FromStr;
    ///
    /// let ok = ["4x Money", "2x Food", "3xSoil"];
    /// let err = ["3 x Money"];
    ///
    /// let ok: Vec<_> = ok.iter().map(|s| Ware::from_str(s)).collect();
    /// let err: Vec<_> = err.iter().map(|s| Ware::from_str(s)).collect();
    ///
    /// assert_eq!(ok, vec![Ok(Ware::new(WareType::Money, 4)), Ok(Ware::new(WareType::Food, 2)), Ok(Ware::new(WareType::Soil, 3))]);
    /// assert_eq!(err.iter().filter(|s| s.is_ok()).next(), None);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let x = match s.find('x') {
            Some(x) => x,
            None => return Err("End of amount delimiter 'x' not found".to_owned()),
        };
        let amount: WareAmount = match s[..x].parse() {
            Ok(amount) => amount,
            Err(_) => return Err(format!("Could not parse amount: '{}'", &s[..x])),
        };
        let ware_type: WareType = match s[x + 1..].trim().parse() {
            Ok(ware_type) => ware_type,
            Err(_) => {
                return Err(format!(
                    "Could not parse ware_type: '{}'",
                    s[x + 1..].trim()
                ))
            }
        };
        Ok(Ware::new(ware_type, amount))
    }
}

impl FromStr for Recipe {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let arrow = match s.find("->") {
            Some(arrow) => arrow,
            None => return Err(format!("Missing arrow (->) in recipe declaration")),
        };

        let inputs = s[..arrow].trim();
        let inputs = &inputs[1..inputs.len() - 1];
        let input_results: Vec<_> = inputs.split(";").map(|s| Ware::from_str(s.trim())).collect();
        let mut inputs = Vec::new();
        for result in input_results {
            match result {
                Ok(w) => inputs.push(w),
                Err(e) => return Err(format!("Could not parse ware declarations: {}", e))
            }
        }

        let outputs = s[arrow + 2..].trim();
        let outputs = &outputs[1..outputs.len() - 1];
        let output_results: Vec<_> = outputs.split(";").map(|s| Ware::from_str(s.trim())).collect();
        let mut outputs = Vec::new();
        for result in output_results {
            match result {
                Ok(w) => outputs.push(w),
                Err(e) => return Err(format!("Could not parse ware declarations: {}", e))
            }
        }

        Ok(Recipe::new(inputs, outputs))
    }
}
