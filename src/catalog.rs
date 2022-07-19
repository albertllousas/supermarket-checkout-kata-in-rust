use rust_decimal::prelude::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Price(pub Decimal);


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sku(pub String);

impl Hash for Sku {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Product { pub sku: Sku, pub price: Price }

pub trait Catalog {
    fn find(&self, sku: &Sku) -> Option<Product>;
}

pub struct InMemoryCatalog {
    pub products: HashMap<Sku, Product>
}

impl Catalog for InMemoryCatalog {
    fn find(&self, sku: &Sku) -> Option<Product> {
        let result = self.products.get(&sku).map (|p| p.clone());
        result
    }
}

#[cfg(test)]
#[path = "./catalog_test.rs"]
mod catalog_test;
