pub type Item = &'static str;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ExchangeablePair {
    pub tx: Item,
    pub rx: Item,
}

pub type Quantity = f64;
pub type UUID = u32;

pub struct Price {
    pub ex: ExchangeablePair,
    pub value: Quantity,
}

impl Eq for Price {}

impl Hash for Price {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ex.hash(state)
    }
}

impl PartialEq for Price {
    fn eq(&self, other: &Price) -> bool {
        self.ex == other.ex
    }
}

enum Void {}
