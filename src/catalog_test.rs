#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use crate::catalog::{Catalog, InMemoryCatalog, Price, Product, Sku};
    use std::collections::HashMap;

    #[test]
    fn should_find_a_product() {
        let product_a = Product { sku: Sku("A".to_string()), price: Price(dec!(1.50)) };
        let mut products: HashMap<Sku, Product> = HashMap::new();
        products.insert(product_a.sku.clone(), product_a.clone());
        let catalog = InMemoryCatalog { products };

        let result = catalog.find(&Sku("A".to_string()));

        assert_eq!(result, Some(product_a.clone()));
    }

    #[test]
    fn should_not_find_a_product() {
        let catalog = InMemoryCatalog { products: HashMap::new() };

        let result = catalog.find(&Sku("X".to_string()));

        assert_eq!(result, None);
    }
}
