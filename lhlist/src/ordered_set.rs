use crate::Label;
use crate::{Cons, LabeledValue, Nil};

impl OrderedHSet for Nil {}

impl<H: Label> OrderedHSet for Cons<LabeledValue<H>, Nil> {}

pub trait OrderedHSet: Sized {
    fn prepend<H: Label>(self, h: LabeledValue<H>) -> Cons<LabeledValue<H>, Self> {
        Cons {
            head: h,
            tail: self,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::ordered_set::OrderedHSet;

    #[test]
    fn create_ordered_set() {
        #[label(type=String, crate=crate)]
        struct ProductName;

        let nil = Nil {} ;
        let elem = LabeledValue::<ProductName>::new("Shampoo".to_string());
        // This should not compile once we are done
        let ordered_set = nil.prepend(elem.clone()).prepend(elem);
    }
}
