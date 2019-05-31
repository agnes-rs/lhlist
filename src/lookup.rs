use crate::cons::LVCons;
use crate::label::Labeled;
use crate::relation::{True, False, LabelEq, Member};

/// Lookup a specific element in a list by label.
pub trait LookupElemByLabel<TargetL> {
    /// The type of the returned element
    type Elem;
    /// Returns a reference to the element from the list
    fn elem(&self) -> &Self::Elem;
}

impl<TargetL, L, V, T> LookupElemByLabel<TargetL> for LVCons<L, V, T>
where
    L: LabelEq<TargetL>,
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
    type Elem;
    /// Returns a reference to the element from the list
    fn elem(&self) -> &Self::Elem;
}

impl<TargetL, L, V, T, TailMatch> LookupElemByLabelMatch<TargetL, True, TailMatch>
    for LVCons<L, V, T>
{
    type Elem = Labeled<L, V>;

    fn elem(&self) -> &Self::Elem { &self.head }
}

impl<TargetL, L, V, T> LookupElemByLabelMatch<TargetL, False, True>
    for LVCons<L, V, T>
where
    T: LookupElemByLabel<TargetL>
{
    type Elem = <T as LookupElemByLabel<TargetL>>::Elem;

    fn elem(&self) -> &Self::Elem {
        LookupElemByLabel::<TargetL>::elem(&self.tail)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[label(name = "My Label")]
    #[derive(Debug)]
    struct Label1;

    #[label(dtype = u8)]
    #[derive(Debug)]
    struct Label2;

    #[label]
    #[derive(Debug)]
    struct Label3;

    #[test]
    fn lookup() {
        let list = lcons![Label1, Label2];
        println!("{:?}", LookupElemByLabel::<Label2>::elem(&list));
    }
}
