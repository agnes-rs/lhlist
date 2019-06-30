use crate::Label;
use crate::{Cons, False, LabeledValue, Member, Nil};

impl OrderedHSet for Nil {}

impl<H: Label, T: OrderedHSet> OrderedHSet for Cons<LabeledValue<H>, T> {}

pub trait OrderedHSet: Sized {
    fn prepend<H>(self, h: LabeledValue<H>) -> Cons<LabeledValue<H>, Self>
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

impl<H, T> Union<Cons<LabeledValue<H>, T>> for Nil
where
    H: Label,
    T: OrderedHSet,
{
    type Output = Cons<LabeledValue<H>, T>;

    fn union(self, rhs: Cons<LabeledValue<H>, T>) -> Self::Output {
        rhs
    }
}

impl<H, T> Union<Nil> for Cons<LabeledValue<H>, T>
where
    H: Label,
    T: OrderedHSet,
{
    type Output = Cons<LabeledValue<H>, T>;

    fn union(self, _rhs: Nil) -> Self::Output {
        self
    }
}

impl Union<Nil> for Nil {
    type Output = Nil;

    fn union(self, rhs: Nil) -> Self::Output {
        rhs
    }
}

impl<H1, T1, H2, T2> Union<Cons<LabeledValue<H2>, T2>> for Cons<LabeledValue<H1>, T1>
where
    H1: Label,
    T1: OrderedHSet,
    H2: Label,
    T2: OrderedHSet,
    Self: Member<H2, Output = False>,
    T2: Union<Cons<LabeledValue<H2>, Cons<LabeledValue<H1>, T1>>>,
{
    type Output = <T2 as Union<Cons<LabeledValue<H2>, Cons<LabeledValue<H1>, T1>>>>::Output;

    fn union(self, rhs: Cons<LabeledValue<H2>, T2>) -> Self::Output {
        rhs.tail.union(self.prepend(rhs.head))
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

        let nil = Nil {};
        let name = LabeledValue::<ProductName>::new("Shampoo".to_string());
        let product_id = LabeledValue::<ProductId>::new(10);
        let shelf_id = LabeledValue::<ShelfId>::new(10);
        let shelf_name = LabeledValue::<ShelfName>::new("Home".to_string());
        let ordered_set = nil
            .clone()
            .prepend(name)
            .prepend(product_id)
            .prepend(shelf_id);
        let singleton = nil.prepend(shelf_name);

        ordered_set.union(singleton);
    }
}
