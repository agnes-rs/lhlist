use std::marker::PhantomData;

use crate::label::{Label, Labeled};
use crate::iter::{ConsIterator, ValuesIterator};
use crate::relation::{Bool, Member};

/// The end of a heterogeneous list.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Nil;

/// Main buildling block of a heterogeneous list.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Cons<H, T> {
    /// Value of this element of the list.
    pub head: H,
    /// Remaining elements of the list.
    pub tail: T,
}

impl<Head, Tail> Cons<Head, Tail> {
    /// Returns a iterator over this cons-list.
    pub fn iter<'a>(&'a self) -> ConsIterator<'a, Self> {
        ConsIterator::new(self)
    }
}

/// A cons-list containing a set of labeled values.
pub type LVCons<Label, Tail> = Cons<Labeled<Label>, Tail>;
/// A cons-list containing only labels.
pub type LCons<Label, Tail> = Cons<PhantomData<Label>, Tail>;

impl<Lbl, Tail> Cons<Lbl, Tail> where Lbl: Label {
    /// Returns `true` if target label exists in this list.
    ///
    /// Convenience function for calling
    /// [has_label_typearg](type.LVCons.html#method.has_label_typearg)
    /// without needing to specify type argument. Calling `list.has_label(ExampleLabel)` is
    /// equivalent to `list.has_label_typearg::<ExampleLabel>())`.
    pub fn has_label<TargetL>(&self, _target_label: TargetL) -> bool
    where
        Self: Member<TargetL>
    {
        self.has_label_typearg::<TargetL>()
    }
    /// Returns `true` if target label exists in this list.
    ///
    /// Calling `list.has_label(ExampleLabel)` is equivalent to
    /// `list.has_label_typearg::<ExampleLabel>())`.
    pub fn has_label_typearg<TargetL>(&self) -> bool
    where
        Self: Member<TargetL>
    {
        <Self as Member<TargetL>>::Output::VALUE
    }

    /// Returns a iterator over this labeled cons-list which iterates over the lists' values (i.e.
    /// object of type `Value`).
    pub fn iter_values<'a>(&'a self) -> ValuesIterator<'a, Self> {
        ValuesIterator::new(self)
    }
}

impl Nil {
    /// Returns `false`, since the `Nil` list has no labels. See
    /// [has_label](type.LVCons.html#method.has_label) for more details.
    pub fn has_label<TargetL>(&self, _target_label:TargetL) -> bool { false }
    /// Returns `false`, since the `Nil` list has no labels. See
    /// [has_label_typearg](type.LVCons.html#method.has_label_typearg) for more details.
    pub fn has_label_typearg<TargetL>(&self) -> bool { false }

    /// Returns an empty [ConsIterator](iter/struct.ConsIterator.html).
    pub fn iter<'a>(&'a self) -> ConsIterator<'a, Self> {
        ConsIterator::new(self)
    }
    /// Returns an empty [ValuesIterator](iter/struct.ValuesIterator.html).
    pub fn iter_values<'a>(&'a self) -> ValuesIterator<'a, Self> {
        ValuesIterator::new(self)
    }
}
