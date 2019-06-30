use crate::Label;
use crate::{Cons, LabeledValue, Nil};

impl OrderedHSet for Nil {}

impl<H: Label, T: OrderedHSet> OrderedHSet for Cons<LabeledValue<H>, T> {}

pub trait OrderedHSet: Sized {
    fn prepend<H>(self, h: LabeledValue<H>) -> Cons<LabeledValue<H>, Self>
    where
        H: Label,
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

        let nil = Nil {};
        let elem = LabeledValue::<ProductName>::new("Shampoo".to_string());
        // This should not compile once we are done
        let _ordered_set = nil
            .prepend(elem.clone())
            .prepend(elem.clone())
            .prepend(elem);
    }
}
