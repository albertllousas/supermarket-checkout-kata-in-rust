use crate::catalog::{Catalog, Price, Sku, Product};
use crate::checkout::CheckoutError::ItemNotFound;
use rust_decimal_macros::dec;

pub struct Checkout<T:Catalog> {
    pub catalog: T,
    products: Vec<Product>
}

#[derive(Debug, PartialEq)]
pub enum CheckoutError {
    ItemNotFound(Sku)
}

pub fn create<T:Catalog>(catalog: T) -> Checkout<T> {
    Checkout { catalog, products: Vec::new()}
}

impl<T:Catalog> Checkout<T> {

    pub fn scan(&mut self, sku: &Sku) -> Result<Price, CheckoutError> {
        let result = self.catalog.find(sku);
        match result {
            None => Err(ItemNotFound(sku.clone())),
            Some(product) => {
                let price = product.price.clone();
                self.products.push(product);
                Ok(price)
            }
        }
    }

    pub fn remove(&mut self, sku: &Sku) -> () {
        let index = &self.products.iter().position(move |x| x.sku == *sku).unwrap();
        self.products.remove(*index);
    }

    pub fn products(&self) -> &Vec<Product> {
        &self.products
    }

    pub fn total(&self) -> Price {
        let sum = self.products.iter()
            .map(|a| a.price.0)
            .fold(dec!(0), |sum, val| sum + val);
        Price(sum)
    }
}

#[cfg(test)]
#[path = "./checkout_test.rs"]
mod checkout_test;
