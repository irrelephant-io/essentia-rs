use std::ops::Add;

use crate::{
    physics::{Power, Quantity},
    engine::ReactionContext,
    EssenceId,
    FormId
};

use super::substance::SubstanceId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Product {
    Produce(EssenceId, FormId, Quantity),
    Consume(EssenceId, FormId, Quantity),
    Dissolve(EssenceId, FormId, SubstanceId, Quantity),
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
            (Product::Consume(eid1, fid1, qty1), Product::Produce(eid2, fid2, qty2)) => {
                if qty1 > qty2 {
                    Product::Consume(eid1, fid1, qty1 - qty2)
                } else {
                    Product::Produce(eid2, fid2, qty2 - qty1)
                }
            },
            (Product::Produce(eid1, fid1, qty1), Product::Consume(eid2, fid2, qty2)) => {
                if qty1 > qty2 {
                    Product::Produce(
                        eid1,
                        fid1,
                        qty1 - qty2
                    )
                } else {
                    Product::Consume(eid2, fid2, qty2 - qty1)
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