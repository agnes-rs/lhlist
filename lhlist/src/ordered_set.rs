use crate::{Cons, LabeledValue, Nil, Member, False};
use crate::{Label};

impl OrderedHSet for Nil {}

impl<H: Label, T: OrderedHSet> OrderedHSet for Cons<LabeledValue<H>, T> {}

pub trait OrderedHSet: Sized {
    fn prepend<H>(self, h: LabeledValue<H>) -> Cons<LabeledValue<H>, Self>
    where
        H: Label,
        Self: Member<H, Output=False>,
    {
        Cons {
            head: h,
            tail: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ordered_set::OrderedHSet;
    use crate::*;

    #[test]
    fn create_ordered_set() {
        #[label(type=String, crate=crate)]
        struct ProductName;

        #[label(type=u8, crate=crate)]
        struct ProductId;

        let nil = Nil {};
        let elem = LabeledValue::<ProductName>::new("Shampoo".to_string());
        let another_elem = LabeledValue::<ProductId>::new(10);
        let _ordered_set = nil
            .prepend(elem)
            .prepend(another_elem);
    }
}
