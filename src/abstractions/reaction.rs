use std::ops::Add;

use crate::{
    engine::ReactionContext,
    physics::{Power, Quantity},
    EssenceId, FormId,
};

use super::substance::SubstanceId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Product {
    Produce(EssenceId, FormId, Quantity),
    Consume(EssenceId, FormId, Quantity),
    Dissolve(EssenceId, FormId, SubstanceId, Quantity),
    Precipitate(EssenceId, FormId, SubstanceId, Quantity),
    Thermal(Power),
}

impl Product {
    #[cfg(test)]
    fn assert_valid_op(self, rhs: Product) {
        match (self, rhs) {
            (Product::Produce(eid1, fid1, _), Product::Produce(eid2, fid2, _)) => {
                if eid1 != eid2 || fid1 != fid2 {
                    panic!("Invalid addition! Products don't match");
                }
            }
            (Product::Consume(eid1, fid1, _), Product::Consume(eid2, fid2, _)) => {
                if eid1 != eid2 || fid1 != fid2 {
                    panic!("Invalid addition! Products don't match");
                }
            }
            (Product::Produce(eid1, fid1, _), Product::Consume(eid2, fid2, _)) => {
                if eid1 != eid2 || fid1 != fid2 {
                    panic!("Invalid addition! Products don't match");
                }
            }
            (Product::Consume(eid1, fid1, _), Product::Produce(eid2, fid2, _)) => {
                if eid1 != eid2 || fid1 != fid2 {
                    panic!("Invalid addition! Products don't match");
                }
            }
            (Product::Dissolve(eid1, fid1, sid1, _), Product::Dissolve(eid2, fid2, sid2, _)) => {
                if eid1 != eid2 || fid1 != fid2 || sid1 != sid2 {
                    panic!("Invalid addition! Products don't match");
                }
            }
            (
                Product::Precipitate(eid1, fid1, sid1, _),
                Product::Precipitate(eid2, fid2, sid2, _),
            ) => {
                if eid1 != eid2 || fid1 != fid2 || sid1 != sid2 {
                    panic!("Invalid addition! Products don't match");
                }
            }
            (Product::Dissolve(eid1, fid1, sid1, _), Product::Precipitate(eid2, fid2, sid2, _)) => {
                if eid1 != eid2 || fid1 != fid2 || sid1 != sid2 {
                    panic!("Invalid addition! Products don't match");
                }
            }
            (Product::Precipitate(eid1, fid1, sid1, _), Product::Dissolve(eid2, fid2, sid2, _)) => {
                if eid1 != eid2 || fid1 != fid2 || sid1 != sid2 {
                    panic!("Invalid addition! Products don't match");
                }
            }
            _ => {}
        }
    }

    #[cfg(not(test))]
    fn assert_valid_op(self, _rhs: Product) {}
}

impl Add<Product> for Product {
    type Output = Product;

    fn add(self, rhs: Product) -> Self::Output {
        self.assert_valid_op(rhs);
        match (self, rhs) {
            (Product::Thermal(p1), Product::Thermal(p2)) => Product::Thermal(p1 + p2),
            (Product::Produce(eid1, fid1, qty1), Product::Produce(_, _, qty2)) => {
                Product::Produce(eid1, fid1, qty1 + qty2)
            }
            (Product::Consume(eid1, fid1, qty1), Product::Produce(_, _, qty2)) => {
                if qty1 > qty2 {
                    Product::Consume(eid1, fid1, qty1 - qty2)
                } else {
                    Product::Produce(eid1, fid1, qty2 - qty1)
                }
            }
            (Product::Produce(eid1, fid1, qty1), Product::Consume(_, _, qty2)) => {
                if qty1 > qty2 {
                    Product::Produce(eid1, fid1, qty1 - qty2)
                } else {
                    Product::Consume(eid1, fid1, qty2 - qty1)
                }
            }
            (Product::Dissolve(eid1, fid1, sid1, qty1), Product::Dissolve(_, _, _, qty2)) => {
                Product::Dissolve(eid1, fid1, sid1, qty1 + qty2)
            }
            (Product::Precipitate(eid1, fid1, sid1, qty1), Product::Precipitate(_, _, _, qty2)) => {
                Product::Precipitate(eid1, fid1, sid1, qty1 + qty2)
            }
            (Product::Dissolve(eid1, fid1, sid1, qty1), Product::Precipitate(_, _, _, qty2)) => {
                if qty1 > qty2 {
                    Product::Dissolve(eid1, fid1, sid1, qty1 - qty2)
                } else {
                    Product::Precipitate(eid1, fid1, sid1, qty2 - qty1)
                }
            }
            (Product::Precipitate(eid1, fid1, sid1, qty1), Product::Dissolve(_, _, _, qty2)) => {
                if qty1 > qty2 {
                    Product::Precipitate(eid1, fid1, sid1, qty1 - qty2)
                } else {
                    Product::Dissolve(eid1, fid1, sid1, qty2 - qty1)
                }
            }
            _ => panic!("Mismatch in product types when adding."),
        }
    }
}

pub trait Reaction {
    fn get_priority(&self) -> u8;
    fn react(&self, context: &ReactionContext) -> Vec<Product>;
}
