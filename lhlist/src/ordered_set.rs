use crate::Label;
use crate::{Cons, False, Member, Nil};

impl OrderedHSet for Nil {}

impl<H, T> OrderedHSet for Cons<H, T>
where
    H: Label,
    T: OrderedHSet + Member<H, Output = False>,
{}

pub trait OrderedHSet: Sized {
    fn prepend<H>(self, h: H) -> Cons<H, Self>
    where
        H: Label,
        Self: Member<H, Output = False>,
    {
        Cons {
            head: h,
            tail: self,
        }
    }
}

pub trait Union<Rhs: OrderedHSet> {
    type Output: OrderedHSet;

    fn union(self, rhs: Rhs) -> Self::Output
    where
        Self: OrderedHSet;
}

impl<Rhs> Union<Rhs> for Nil
where
    Rhs: OrderedHSet
{
    type Output = Rhs;

    fn union(self, rhs: Rhs) -> Self::Output {
        rhs
    }
}

impl<H, T, Rhs> Union<Rhs> for Cons<H, T>
where
    H: Label,
    T: OrderedHSet + Union<Rhs>,
    Rhs: OrderedHSet,
    Cons<H, <T as Union<Rhs>>::Output>: OrderedHSet,
{
    type Output = Cons<H, <T as Union<Rhs>>::Output>;

    fn union(self, rhs: Rhs) -> Self::Output {
        Cons {
            head: self.head,
            tail: self.tail.union(rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ordered_set::{OrderedHSet, Union};
    use crate::*;

    #[test]
    fn create_ordered_set() {
        #[label(type=String, crate=crate)]
        struct ProductName;

        #[label(type=u8, crate=crate)]
        struct ProductId;

        #[label(type=u8, crate=crate)]
        struct ShelfId;

        #[label(type=String, crate=crate)]
        struct ShelfName;

        #[label(type=String, crate=crate)]
        struct StoreName;

        #[label(type=f64, crate=crate)]
        struct Price;

        let nil = Nil {};
        let name = LabeledValue::<ProductName>::new("Shampoo".to_string());
        let product_id = LabeledValue::<ProductId>::new(10);
        let shelf_id = LabeledValue::<ShelfId>::new(10);
        let shelf_name = LabeledValue::<ShelfName>::new("Home".to_string());
        let store_name = LabeledValue::<StoreName>::new("X".to_string());
        let price = LabeledValue::<Price>::new(12.0);
        let ordered_set = nil
            .clone()
            .prepend(name)
            .prepend(product_id)
            .prepend(shelf_id);
        let singleton = nil.clone().prepend(shelf_name);
        let another_set = nil.clone().prepend(store_name).prepend(price);

        ordered_set.union(singleton).union(another_set);
    }
}
