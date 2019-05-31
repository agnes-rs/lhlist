use typenum::{B0, B1, IsEqual};

use crate::cons::{Nil, LVCons};
use crate::label::Label;

/// Marker struct signifying `true`.
pub struct True;
/// Marker struct signifying `false`.
pub struct False;

/// Trait for types that signify `true` or `false`.
pub trait Bool {
    /// `true` or `false` value
    const VALUE: bool;
}
impl Bool for True {
    const VALUE: bool = true;
}
impl Bool for False {
    const VALUE: bool = false;
}

/// Conversion trait for types that have a logical `true` or `false` meaning
pub trait ToBool {
    /// Conversion output (typically either [True](struct.True.html) or [False](struct.False.html))
    type Output: Bool;
}
impl ToBool for B1 { type Output = True; }
impl ToBool for B0 { type Output = False; }

/// Label equality.
///
/// Checks whether two types that implement [Label](trait.Label.html) are the same.
pub trait LabelEq<L> {
    /// [True](struct.True.html) if labels are equal, [False](struct.False.html) otherwise.
    type Output: Bool;
}

impl<L, M> LabelEq<M> for L
where
    L: Label,
    M: Label,
    L::Uid: IsEqual<M::Uid>,
    <L::Uid as IsEqual<M::Uid>>::Output: ToBool,
{
    type Output = <<L::Uid as IsEqual<M::Uid>>::Output as ToBool>::Output;
}


/// Check to see if a target label is a list member.
pub trait Member<TargetL> {
    /// [True](struct.True.html) if `TargetL` is a member, [False](struct.False.html) otherwise.
    type Output: Bool;
}

impl<TargetL> Member<TargetL> for Nil {
    type Output = False;
}
impl<TargetL, L, V, T> Member<TargetL> for LVCons<L, V, T>
where
    L: LabelEq<TargetL>,
    Self: MemberMatch<TargetL, <L as LabelEq<TargetL>>::Output>,
{
    type Output = <Self as MemberMatch<TargetL, <L as LabelEq<TargetL>>::Output>>::Output;
}

/// Helper trait for [Member](trait.Member.html).
pub trait MemberMatch<L, HeadMatch> {
    /// [True](struct.True.html) if `TargetL` is a member, [False](struct.False.html) otherwise.
    type Output: Bool;
}

impl<TargetL, V, T> MemberMatch<TargetL, True> for LVCons<TargetL, V, T>
{
    type Output = True;
}
impl<TargetL, L, V, T> MemberMatch<TargetL, False> for LVCons<L, V, T>
where
    T: Member<TargetL>,
{
    type Output = <T as Member<TargetL>>::Output;
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
    fn label_eq() {
        assert!(<Label1 as LabelEq<Label1>>::Output::VALUE);
        assert!(<Label2 as LabelEq<Label2>>::Output::VALUE);
        assert!(<Label3 as LabelEq<Label3>>::Output::VALUE);

        assert!(!<Label1 as LabelEq<Label2>>::Output::VALUE);
        assert!(!<Label1 as LabelEq<Label3>>::Output::VALUE);

        assert!(!<Label2 as LabelEq<Label1>>::Output::VALUE);
        assert!(!<Label2 as LabelEq<Label3>>::Output::VALUE);

        assert!(!<Label3 as LabelEq<Label1>>::Output::VALUE);
        assert!(!<Label3 as LabelEq<Label2>>::Output::VALUE);
    }

    #[test]
    fn member() {
        // type-based member testing
        type TestList = LCons<Label1, LCons<Label2, Nil>>;
        assert!(<TestList as Member<Label1>>::Output::VALUE);
        assert!(<TestList as Member<Label2>>::Output::VALUE);
        assert!(!<TestList as Member<Label3>>::Output::VALUE);

        // value-based member testing
        let list = lcons![Label1, Label2];
        assert!(list.has_label(Label1));
        assert!(list.has_label(Label2));
        assert!(!list.has_label(Label3));
    }
}
