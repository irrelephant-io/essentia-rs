use std::ops::Add;

use crate::{
    abstractions::physics::Power,
    engine::ReactionContext,
    physics::Quantity, SubstanceData
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Product {
    Produce(SubstanceData),
    Dissolve(u16, u16, Quantity),
    Consume(u16, Quantity),
    Thermal(Power),
}

impl Add<Product> for Product {
    type Output = Product;
    
    fn add(self, rhs: Product) -> Self::Output {
        match (self, rhs) {
            (Product::Thermal(p1), Product::Thermal(p2)) => Product::Thermal(p1 + p2),
            (Product::Produce(s1), Product::Produce(s2)) => Product::Produce(
                SubstanceData {
                    substance_id: s1.substance_id,
                    essence_id: s1.essence_id,
                    form_id: s1.form_id,
                    quantity: s1.quantity + s2.quantity
                }
            ),
            (Product::Consume(id1, qty1), Product::Produce(s2)) => {
                if qty1 > s2.quantity {
                    Product::Consume(id1, qty1 - s2.quantity)
                } else {
                    Product::Produce(
                        SubstanceData {
                            substance_id: s2.substance_id,
                            essence_id: s2.essence_id,
                            form_id: s2.form_id,
                            quantity: s2.quantity - qty1
                        }
                    )
                }
            },
            (Product::Produce(s1), Product::Consume(id2, qty2)) => {
                if s1.quantity > qty2 {
                    Product::Produce(
                        SubstanceData {
                            substance_id: s1.substance_id,
                            essence_id: s1.essence_id,
                            form_id: s1.form_id,
                            quantity: s1.quantity - qty2
                        }
                    )
                } else {
                    Product::Consume(id2, qty2 - s1.quantity)
                }
            },
            _ => panic!("Mismatch in product types when adding.")
        }
    }
}

pub trait Reaction {
    fn get_priority(&self) -> u8;
    fn react(&self, context: &ReactionContext) -> Vec::<Product>;
}