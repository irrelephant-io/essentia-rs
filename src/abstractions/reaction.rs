use std::ops::Add;

use crate::{
    abstractions::physics::Power,
    engine::ReactionContext,
    physics::Quantity
};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Product {
    Produce(u16, u16, Quantity),
    Dissolve(u16, u16, Quantity),
    Consume(u16, Quantity),
    Thermal(Power),
}

impl Add<Product> for Product {
    type Output = Product;
    
    fn add(self, rhs: Product) -> Self::Output {
        match (self, rhs) {
            (Product::Thermal(p1), Product::Thermal(p2)) => Product::Thermal(p1 + p2),
            (Product::Produce(eid1, fid1, qty1), Product::Produce(_, _, qty2)) => Product::Produce(
                eid1,
                fid1,
                qty1 + qty2
            ),
            (Product::Consume(id1, qty1), Product::Produce(eid2, fid2, qty2)) => {
                if qty1 > qty2 {
                    Product::Consume(id1, qty1 - qty2)
                } else {
                    Product::Produce(eid2, fid2, qty2 - qty1)
                }
            },
            (Product::Produce(eid1, fid1, qty1), Product::Consume(id2, qty2)) => {
                if qty1 > qty2 {
                    Product::Produce(
                        eid1,
                        fid1,
                        qty1 - qty2
                    )
                } else {
                    Product::Consume(id2, qty2 - qty1)
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