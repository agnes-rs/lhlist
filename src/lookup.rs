use crate::cons::LVCons;
use crate::label::{Label, LabeledValue};
use crate::relation::{True, False, LabelEq, Member};

/// Lookup a specific element in a list by label.
pub trait LookupElemByLabel<TargetL> {
    /// The type of the returned element
    type Elem;
    /// Returns a reference to the element from the list
    fn elem(&self) -> &Self::Elem;
}

impl<TargetL, L, T> LookupElemByLabel<TargetL> for LVCons<L, T>
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
    type Elem;
    /// Returns a reference to the element from the list
    fn elem(&self) -> &Self::Elem;
}

impl<TargetL, L, T, TailMatch> LookupElemByLabelMatch<TargetL, True, TailMatch>
    for LVCons<L, T>
where
    L: Label
{
    type Elem = LabeledValue<L>;

    fn elem(&self) -> &Self::Elem { &self.head }
}

impl<TargetL, L, T> LookupElemByLabelMatch<TargetL, False, True>
    for LVCons<L, T>
where
    L: Label,
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

    #[label(name="My Label", type=String)]
    struct Label1;

    #[label(type=u8)]
    struct Label2;

    #[label]
    struct Label3;

    #[test]
    fn lookup() {
        let list = lhlist![
            Label1 = "first value".to_string(),
            Label2 = 2
        ];
        println!("{:?}", LookupElemByLabel::<Label2>::elem(&list));
    }
}
