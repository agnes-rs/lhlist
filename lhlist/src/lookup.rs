use crate::cons::Cons;
use crate::label::Label;
use crate::relation::{True, False, LabelEq, Member};

/// Lookup a specific element in a list by label.
pub trait LookupElemByLabel<TargetL> {
    /// The type of the returned element
    type Elem: ?Sized;
    /// Returns a reference to the element from the list
    fn elem(&self) -> &Self::Elem;
}

impl<TargetL, L, T> LookupElemByLabel<TargetL> for Cons<L, T>
where
    L: Label + LabelEq<TargetL>,
    T: Member<TargetL>,
    Self: LookupElemByLabelMatch<
        TargetL,
        <L as LabelEq<TargetL>>::Output,
        <T as Member<TargetL>>::Output,
    >,
{
    type Elem = <Self as LookupElemByLabelMatch<
        TargetL,
        <L as LabelEq<TargetL>>::Output,
        <T as Member<TargetL>>::Output
    >>::Elem;

    fn elem(&self) -> &Self::Elem {
        LookupElemByLabelMatch::<
            TargetL,
            <L as LabelEq<TargetL>>::Output,
            <T as Member<TargetL>>::Output,
        >::elem(self)
    }
}

/// Helper trait for [LookupElemByLabel](trait.LookupElemByLabel.html).
pub trait LookupElemByLabelMatch<L, LabelMatch, TailMatch> {
    /// The type of the returned element
    type Elem: ?Sized;
    /// Returns a reference to the element from the list
    fn elem(&self) -> &Self::Elem;
}

// head matches
impl<TargetL, L, T, TailMatch> LookupElemByLabelMatch<TargetL, True, TailMatch>
    for Cons<L, T>
where
    L: Label
{
    type Elem = L;

    fn elem(&self) -> &Self::Elem { &self.head }
}

// head doesn't match but tail does
impl<TargetL, L, T> LookupElemByLabelMatch<TargetL, False, True>
    for Cons<L, T>
where
    L: Label,
    T: LookupElemByLabel<TargetL>
{
    type Elem = <T as LookupElemByLabel<TargetL>>::Elem;

    fn elem(&self) -> &Self::Elem {
        LookupElemByLabel::<TargetL>::elem(&self.tail)
    }
}

/// Lookup a specific mutable element in a list by label.
pub trait LookupElemByLabelMut<TargetL>: LookupElemByLabel<TargetL> {
    /// Returns a mutable reference to the element from the list
    fn elem_mut(&mut self) -> &mut Self::Elem;
}

impl<TargetL, L, T> LookupElemByLabelMut<TargetL> for Cons<L, T>
where
    L: Label + LabelEq<TargetL>,
    T: Member<TargetL>,
    Self: LookupElemByLabel<TargetL>,
    Self: LookupElemByLabelMutMatch<
        TargetL,
        <L as LabelEq<TargetL>>::Output,
        <T as Member<TargetL>>::Output,
        Elem=<Self as LookupElemByLabel<TargetL>>::Elem,
    >,
{
    fn elem_mut(&mut self) -> &mut Self::Elem {
        LookupElemByLabelMutMatch::<
            TargetL,
            <L as LabelEq<TargetL>>::Output,
            <T as Member<TargetL>>::Output,
        >::elem_mut(self)
    }
}

/// Helper trait for [LookupElemByLabelMut](trait.LookupElemByLabelMut.html).
pub trait LookupElemByLabelMutMatch<L, LabelMatch, TailMatch>:
    LookupElemByLabelMatch<L, LabelMatch, TailMatch>
{
    /// Returns a mutable reference to the element from the list
    fn elem_mut(&mut self) -> &mut Self::Elem;
}

// head matches
impl<TargetL, L, T, TailMatch> LookupElemByLabelMutMatch<TargetL, True, TailMatch>
    for Cons<L, T>
where
    L: Label
{
    fn elem_mut(&mut self) -> &mut Self::Elem { &mut self.head }
}

// head doesn't match but tail does
impl<TargetL, L, T> LookupElemByLabelMutMatch<TargetL, False, True>
    for Cons<L, T>
where
    L: Label,
    T: LookupElemByLabelMut<TargetL>
{
    fn elem_mut(&mut self) -> &mut Self::Elem {
        LookupElemByLabelMut::<TargetL>::elem_mut(&mut self.tail)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[label(name="My Label", type=String, crate=crate)]
    struct Label1;

    #[label(type=u8, crate=crate)]
    struct Label2;

    #[label(type=&'static str, crate=crate)]
    struct Label3;

    #[test]
    fn lookup() {
        let list = lhlist![
            Label1 = "first value".to_string(),
            Label2 = 2,
            Label3 = "third value",
        ];
        println!("{:?}", LookupElemByLabel::<Label1>::elem(&list));
        println!("{:?}", LookupElemByLabel::<Label2>::elem(&list));
        println!("{:?}", LookupElemByLabel::<Label3>::elem(&list));
    }
}
