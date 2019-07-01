use crate::Label;
use crate::{Cons, False, Member, Nil};

/// Nil corresponds to an empty set.
impl OrderedHSet for Nil {}

/// A labeled heterogeneous list is an `OrderedHSet` if:
/// - the head `H` is labeled;
/// - the tail `T` is a set;
/// - `T` does not contain any element with same **label** of `H`.
impl<H, T> OrderedHSet for Cons<H, T>
where
    H: Label,
    T: OrderedHSet + Member<H, Output = False>,
{}

/// An `OrderedHSet` is a labeled heterogeneous list that does not contain
/// elements with the same label.
pub trait OrderedHSet {
    /// It creates a new set by prepending `h` to `self`.
    ///
    /// If there is another element in `self` with the same
    /// label as `h`, it fails at compile-time.
    fn prepend<H>(self, h: H) -> Cons<H, Self>
    where
        H: Label,
        Self: Member<H, Output = False> + Sized,
    {
        Cons {
            head: h,
            tail: self,
        }
    }
}

/// The union operation for [OrderedHSet](trait.OrderedHSet.html)s.
///
/// It is not commutative: the order of the elements in the final
/// depends on the order of the operands.
pub trait Union<Rhs: OrderedHSet> {
    /// The result type of the union operation.
    type Output: OrderedHSet;

    /// It returns the union of two [OrderedHSet](trait.OrderedHSet.html)s.
    ///
    /// The elements of `Self` are added at the beginning of the resulting
    /// [OrderedHSet](trait.OrderedHSet.html).
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

        let name = LabeledValue::<ProductName>::new("Shampoo".to_string());
        let product_id = LabeledValue::<ProductId>::new(10);
        let shelf_id = LabeledValue::<ShelfId>::new(10);
        let shelf_name = LabeledValue::<ShelfName>::new("Home".to_string());
        let store_name = LabeledValue::<StoreName>::new("X".to_string());
        let price = LabeledValue::<Price>::new(12.0);
        let ordered_set = Nil
            .prepend(name)
            .prepend(product_id)
            .prepend(shelf_id);
        let singleton = Nil.prepend(shelf_name);
        let another_set = Nil.prepend(store_name).prepend(price);

        ordered_set.union(singleton).union(another_set);
    }
}
