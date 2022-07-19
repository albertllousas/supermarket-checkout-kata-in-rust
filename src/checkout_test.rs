#[cfg(test)]
mod tests {
    use mockall::*;
    use mockall::predicate::*;
    use rust_decimal_macros::dec;

    use crate::catalog::{Catalog, Price, Product, Sku};
    use crate::checkout::Checkout;
    use crate::checkout::CheckoutError::ItemNotFound;
    use crate::checkout::create;

    mock! {
        pub Catalog {}
        impl Catalog for Catalog {fn find(&self, sku: &Sku) -> Option<Product>;}
    }

    #[test]
    fn should_fail_scanning_a_non_existent_product_in_the_catalog() {
        let mut catalog = MockCatalog::new();
        let sku = Sku("X".to_string());
        catalog
            .expect_find()
            .with(predicate::eq(sku.clone()))
            .returning(|_| None);
        let mut checkout = create(catalog);

        let result = checkout.scan(&sku);

        assert_eq!(result, Err(ItemNotFound(sku.clone())));
    }

    #[test]
    fn should_scan_an_existent_product_of_the_catalog() {
        let mut catalog = MockCatalog::new();
        let sku = Sku("A".to_string());
        catalog
            .expect_find()
            .with(predicate::eq(Sku("A".to_string())))
            .returning(|_| Some(Product { sku: Sku("A".to_string()), price: Price(dec!(1.50)) }));
        let mut checkout = create(catalog);

        let result = checkout.scan(&sku);

        assert_eq!(result, Ok(Price(dec!(1.50))));
    }

    #[test]
    fn should_add_scanned_items_to_the_list_of_current_products() {
        let mut catalog = MockCatalog::new();
        let product_a = Product { sku: Sku("A".to_string()), price: Price(dec!(1.50)) };
        let copy_a = product_a.clone();
        let product_b = Product { sku: Sku("B".to_string()), price: Price(dec!(2.50)) };
        let copy_b = product_b.clone();
        catalog
            .expect_find()
            .with(predicate::eq(product_a.sku.clone()))
            .returning(move |_| Some(product_a.clone()));
        catalog
            .expect_find()
            .with(predicate::eq(product_b.sku.clone()))
            .returning(move |_| Some(product_b.clone()));
        let mut checkout = create(catalog);
        checkout.scan(&Sku("A".to_string()));
        checkout.scan(&Sku("B".to_string()));

        let result = checkout.products();

        assert_eq!(result, &vec!(copy_a, copy_b));
    }

    #[test]
    fn should_remove_an_already_scanned_item_from_current_products() {
        let product_a = Product { sku: Sku("A".to_string()), price: Price(dec!(1.50)) };
        let product_b = Product { sku: Sku("B".to_string()), price: Price(dec!(2.50)) };
        let copy_b = product_b.clone();
        let mut checkout = Checkout { catalog: MockCatalog::new(), products: vec!(product_a, product_b) };

        checkout.remove(&Sku("A".to_string()));

        assert_eq!(checkout.products(), &vec!(copy_b));
    }

    #[test]
    fn should_calculate_the_total_price_of_the_products() {
        let product_a = Product { sku: Sku("A".to_string()), price: Price(dec!(1.50)) };
        let product_b = Product { sku: Sku("B".to_string()), price: Price(dec!(2.50)) };
        let product_c = Product { sku: Sku("C".to_string()), price: Price(dec!(0.50)) };
        let checkout = Checkout {
            catalog: MockCatalog::new(),
            products: vec!(product_a, product_b, product_c),
        };

        let result = checkout.total();

        assert_eq!(result, Price(dec!(4.50)));
    }
}
