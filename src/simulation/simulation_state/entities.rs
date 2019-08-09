#[derive(Debug, Clone, new)]
pub struct BuyerProperties {
    max_price: i32,
}

#[derive(Debug, Clone, new)]
pub struct SellerProperties {
    min_price: i32,
}

#[derive(Debug, Clone, new, Getters, MutGetters)]
pub struct Buyer {
    #[get = "pub"] #[get_mut = "pub"] expected_price: i32,
    #[get = "pub"] #[get_mut = "pub"] properties: BuyerProperties,
}

#[derive(Debug, Clone, new, Getters, MutGetters)]
pub struct Seller {
    #[get = "pub"] #[get_mut = "pub"] expected_price: i32,
    #[get = "pub"] #[get_mut = "pub"] properties: SellerProperties,
}

impl Buyer {
    pub fn increase_expected_price(&mut self) {
        if self.expected_price < self.properties.max_price {
            self.expected_price += 1;
        }
    }

    pub fn decrease_expected_price(&mut self) {
        self.expected_price -= 1;
    }
}

impl Seller {
    pub fn increase_expected_price(&mut self) {
        self.expected_price += 1;
    }

    pub fn decrease_expected_price(&mut self) {
        if self.expected_price > self.properties.min_price {
            self.expected_price -= 1;
        }
    }
}

impl From<BuyerProperties> for Buyer {
    fn from(properties: BuyerProperties) -> Self {
        Buyer {expected_price: properties.max_price, properties}
    }
}

impl From<SellerProperties> for Seller {
    fn from(properties: SellerProperties) -> Self {
        Seller {expected_price: properties.min_price, properties}
    }
}

impl From<i32> for BuyerProperties {
    fn from(max_price: i32) -> Self {
        Self {max_price}
    }
}

impl From<i32> for SellerProperties {
    fn from(min_price: i32) -> Self {
        Self {min_price}
    }
}