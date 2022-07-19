use checkout_kata_in_rust::{checkout, catalog};
use rust_decimal_macros::dec;
use crate::checkout::{Checkout, create};
use crate::catalog::{Catalog, InMemoryCatalog, Price, Product, Sku};
use std::collections::HashMap;

#[test]
fn should_checkout() {
    let product_a = Product { sku: Sku("A".to_string()), price: Price(dec!(1.99)) };
    let product_b = Product { sku: Sku("B".to_string()), price: Price(dec!(2.50)) };
    let product_c = Product { sku: Sku("C".to_string()), price: Price(dec!(3.00)) };
    let product_d = Product { sku: Sku("D".to_string()), price: Price(dec!(4.35)) };
    let mut products: HashMap<Sku, Product> = HashMap::new();
    products.insert(product_a.sku.clone(), product_a.clone());
    products.insert(product_b.sku.clone(), product_b.clone());
    products.insert(product_c.sku.clone(), product_c.clone());
    products.insert(product_d.sku.clone(), product_d.clone());
    let catalog = InMemoryCatalog { products };
    let mut checkout = create(catalog);
    checkout.scan(&product_a.sku);
    checkout.scan(&product_a.sku);
    checkout.scan(&product_b.sku);
    checkout.scan(&product_d.sku);

    let result = checkout.total();

    assert_eq!(result, Price(dec!(10.83)));
}