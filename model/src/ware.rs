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

#[derive(Default, Clone, Debug, Eq, PartialEq)]
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

    pub fn is_money(&self) -> bool {
        self.ware_type().is_money()
    }
}

impl From<(WareType, WareAmount)> for Ware {
    fn from((ware_type, amount): (WareType, u32)) -> Self {
        Ware::new(ware_type, amount)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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
    /// Pushes the given ware to this store.
    pub fn push_ware(&mut self, ware: Ware) {
        trace!("Pushing {}", ware);

        if ware.amount() == 0 {
            return;
        }

        if let Some(amount) = self.wares.get_mut(&ware.ware_type()) {
            *amount += ware.amount();
        } else {
            self.wares.insert(ware.ware_type(), ware.amount());
        }
    }

    /// Pops the given ware from this store.
    /// Fails if the ware store does not contain enough wares to pop.
    ///
    /// Returns Ok containing the popped ware if successful, Err otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use model::ware::*;
    ///
    /// let mut store = WareStore::new();
    /// store.push_ware(Ware::new(WareType::Money, 5));
    /// assert_eq!(5, store.ware_amount(WareType::Money));
    /// store.push_ware(Ware::new(WareType::Money, 3));
    /// assert_eq!(8, store.ware_amount(WareType::Money));
    /// store.pop_ware(Ware::new(WareType::Money, 4));
    /// assert_eq!(4, store.ware_amount(WareType::Money));
    /// ```
    pub fn pop_ware(&mut self, ware: Ware) -> Result<Ware, ()> {
        trace!("Popping {}", ware);

        if ware.amount() == 0 {
            return Ok(ware);
        }

        if let Some(amount) = self.wares.get_mut(&ware.ware_type()) {
            if *amount >= ware.amount() {
                if *amount > ware.amount() {
                    *amount -= ware.amount();
                } else {
                    self.wares.remove(&ware.ware_type()).unwrap();
                }
                Ok(ware)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    /// Pops the given ware from this store.
    /// Removes only the amount of ware that is in the store.
    ///
    /// Returns the ware that was popped, and modifies the input to the amount that was not popped.
    ///
    /// # Example
    ///
    /// ```
    /// use model::ware::*;
    ///
    /// let mut store = WareStore::new();
    /// store.push_ware(Ware::new(WareType::Money, 5));
    /// assert_eq!(5, store.ware_amount(WareType::Money));
    ///
    /// let mut pop = Ware::new(WareType::Money, 4);
    /// let popped = store.pop_ware_max(&mut pop);
    /// assert_eq!(1, store.ware_amount(WareType::Money));
    /// assert_eq!(0, pop.amount());
    /// assert_eq!(4, popped.amount());
    ///
    /// let mut pop = Ware::new(WareType::Money, 3);
    /// let popped = store.pop_ware_max(&mut pop);
    /// assert_eq!(0, store.ware_amount(WareType::Money));
    /// assert_eq!(2, pop.amount());
    /// assert_eq!(1, popped.amount());
    /// ```
    pub fn pop_ware_max(&mut self, ware: &mut Ware) -> Ware {
        trace!("Popping max {}", ware);

        if let Some(amount) = self.wares.get_mut(&ware.ware_type()) {
            if *amount > ware.amount() {
                let pop_amount = ware.amount();
                *amount -= pop_amount;
                *ware.amount_mut() = 0;
                Ware::new(ware.ware_type(), pop_amount)
            } else {
                let pop_amount = *amount;
                self.wares.remove(&ware.ware_type());
                *ware.amount_mut() -= pop_amount;
                Ware::new(ware.ware_type(), pop_amount)
            }
        } else {
            Ware::new(ware.ware_type(), 0)
        }
    }

    /// Pops the wares in the given ware store from this ware store.
    /// Fails, if not all wares can be popped completely.
    /// If it fails, this ware store remains untouched.
    ///
    /// Returns the removed wares if successful.
    ///
    /// # Example
    ///
    /// ```
    /// use model::ware::*;
    ///
    /// let mut store = WareStore::new();
    /// store.push_ware(Ware::new(WareType::Money, 13));
    /// store.push_ware(Ware::new(WareType::Water, 3));
    /// store.push_ware(Ware::new(WareType::Food, 5));
    /// let mut pop = WareStore::new();
    /// pop.push_ware(Ware::new(WareType::Money, 11));
    /// pop.push_ware(Ware::new(WareType::Water, 3));
    /// pop.push_ware(Ware::new(WareType::Food, 3));
    /// let mut popped = WareStore::new();
    /// popped.push_ware(Ware::new(WareType::Money, 2));
    /// popped.push_ware(Ware::new(WareType::Food, 2));
    /// assert_eq!(Ok(pop.clone()), store.pop_wares(pop.clone()));
    /// assert_eq!(popped, store);
    ///
    /// assert_eq!(Err(()), store.pop_wares(pop));
    /// ```
    pub fn pop_wares(&mut self, wares: WareStore) -> Result<WareStore, ()> {
        trace!("Popping {}", wares);

        for ware_type in wares.iter_ware_types() {
            if self.ware_amount(ware_type) < wares.ware_amount(ware_type) {
                return Err(());
            }
        }

        for ware_type in wares.iter_ware_types() {
            self.pop_ware(wares.get_ware(ware_type).unwrap()).unwrap();
        }

        Ok(wares)
    }

    /// Pops as much as possible from the given wares from this ware store.
    /// The given wares a modified to what was not popped, and the result is what was popped.
    ///
    /// # Example
    ///
    /// ```
    /// use model::ware::*;
    ///
    /// let mut store = WareStore::new();
    /// store.push_ware(Ware::new(WareType::Money, 13));
    /// store.push_ware(Ware::new(WareType::Water, 3));
    /// store.push_ware(Ware::new(WareType::Food, 5));
    /// let mut pop = WareStore::new();
    /// pop.push_ware(Ware::new(WareType::Money, 11));
    /// pop.push_ware(Ware::new(WareType::Water, 4));
    /// pop.push_ware(Ware::new(WareType::Food, 3));
    /// let mut pop_result = WareStore::new();
    /// pop_result.push_ware(Ware::new(WareType::Money, 11));
    /// pop_result.push_ware(Ware::new(WareType::Water, 3));
    /// pop_result.push_ware(Ware::new(WareType::Food, 3));
    /// let mut pop_leftover = WareStore::new();
    /// pop_leftover.push_ware(Ware::new(WareType::Water, 1));
    /// let mut popped = WareStore::new();
    /// popped.push_ware(Ware::new(WareType::Money, 2));
    /// popped.push_ware(Ware::new(WareType::Food, 2));
    /// assert_eq!(pop_result.clone(), store.pop_wares_max(&mut pop));
    /// assert_eq!(popped, store);
    /// assert_eq!(pop_leftover, pop);
    /// ```
    pub fn pop_wares_max(&mut self, wares: &mut WareStore) -> WareStore {
        trace!("Popping max {}", wares);

        let mut popped = WareStore::new();
        for (ware_type, amount) in wares.iter_mut() {
            let mut pop = Ware::from((*ware_type, *amount));
            popped.push_ware(self.pop_ware_max(&mut pop));
            *amount = pop.amount();
        }
        wares.clean();
        popped
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&WareType, &mut WareAmount)> + 'a {
        self.wares.iter_mut()
    }

    fn clean(&mut self) {
        self.wares.retain(|_, v| *v > 0);
    }
}

// Getters
impl WareStore {
    pub fn iter_ware_types<'a>(&'a self) -> impl Iterator<Item = WareType> + 'a {
        self.wares.keys().cloned()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Ware> + 'a {
        self.wares
            .iter()
            .map(|(ware_type, amount)| Ware::new(*ware_type, *amount))
    }

    pub fn ware_amount(&self, ware_type: WareType) -> WareAmount {
        self.wares.get(&ware_type).cloned().unwrap_or(0)
    }

    pub fn get_ware(&self, ware_type: WareType) -> Option<Ware> {
        Some(Ware::new(ware_type, self.wares.get(&ware_type)?.clone()))
    }
}
