use std::collections::HashMap;

use crate::physics::Power;
use crate::reaction::Product;

use super::Essentia;

pub struct ReactionContext<'a> {
    pub engine: &'a Essentia,
    pub pending_products: Vec<Product>
}

impl<'a> ReactionContext<'a> {
    pub fn new(engine: &'a Essentia) -> Self {
        Self {
            engine,
            pending_products: vec![]
        }
    }

    pub fn apply(self, products: Vec<Product>) -> Self {
        let mut thermal_product = Product::Thermal(Power::from(0));
        let mut substance_products = HashMap::<(u16, u16), Product>::new();

        for product in self.pending_products.into_iter().chain(products.into_iter()) {
            match product {
                Product::Thermal(_) => { thermal_product = thermal_product + product; },
                Product::Produce(essence_id, form_id, _) => {
                    substance_products
                        .entry((essence_id, form_id))
                        .and_modify(|e| {
                            let result = *e + product;
                            *e = result;
                        })
                        .or_insert(product);
                },
                Product::Consume(essence_id, form_id, _) => {
                    substance_products
                        .entry((essence_id, form_id))
                        .and_modify(|e| {
                            let result = *e + product;
                            *e = result;
                        })
                        .or_insert(product);
                },
                _ => { todo!("This product is not supported" )}
            }
        }

        let mut products_vec = substance_products
            .into_values()
            .filter(|p| {
                match p {
                    Product::Consume(_, _, qty) => qty.mol != 0,
                    Product::Produce(_, _, qty) => qty.mol != 0,
                    _ => true
                }
            })
            .collect::<Vec<_>>();

        if let Product::Thermal(p) = thermal_product {
            if p.watts != 0 {
                products_vec.push(thermal_product);
            }
        }
        
        ReactionContext {
            engine: self.engine,
            pending_products: products_vec
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{engine::EssentiaBuilder, physics::{Power, Quantity}, reaction::Product};
    use super::ReactionContext;

    #[test]
    pub fn test_applying_empty() {
        let engine_dummy = EssentiaBuilder::new().build();
        let context = ReactionContext::new(&engine_dummy);

        let next_context = context.apply(vec![]);

        assert_eq!(next_context.pending_products.len(), 0);
    }

    #[test]
    pub fn test_squishing_thermals() {
        let engine_dummy = EssentiaBuilder::new().build();
        let context = ReactionContext::new(&engine_dummy);

        let next_context = context.apply(vec![
            Product::Thermal(Power::from(10)),
            Product::Thermal(Power::from(20))
        ]);

        let thermal = next_context.pending_products.last().unwrap();
        assert_eq!(Product::Thermal(Power::from(30)), *thermal);
    }

    #[test]
    pub fn test_neutralizing_thermals() {
        let engine_dummy = EssentiaBuilder::new().build();
        let context = ReactionContext::new(&engine_dummy);

        let next_context = context.apply(vec![
            Product::Thermal(Power::from(10)),
            Product::Thermal(Power::from(-10))
        ]);

        assert_eq!(next_context.pending_products.len(), 0);
    }

    #[test]
    pub fn test_neutralizing_substance() {
        let engine_dummy = EssentiaBuilder::new().build();
        let context = ReactionContext::new(&engine_dummy);

        let next_context = context.apply(vec![
            Product::Produce(0, 0, Quantity::from(5)),
            Product::Consume(0, 0, Quantity::from(5)),
        ]);

        assert_eq!(next_context.pending_products.len(), 0);
    }

    #[test]
    pub fn test_adding_production_and_consumption() {
        let engine_dummy = EssentiaBuilder::new().build();
        let context = ReactionContext::new(&engine_dummy);

        let next_context = context.apply(vec![
            Product::Produce(0, 0, Quantity::from(5)),
            Product::Produce(0, 0, Quantity::from(5)),
            Product::Consume(0, 0, Quantity::from(2)),
            Product::Produce(0, 1, Quantity::from(5))
        ]);

        let expected = vec![
            Product::Produce(0, 0, Quantity::from(8)),
            Product::Produce(0, 1, Quantity::from(5))
        ];

        assert!(
            next_context.pending_products
                .iter()
                .all(|item| expected.contains(item))
        );
    }
}