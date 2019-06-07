use std::marker::PhantomData;

use crate::label::{Label, LabeledValue};
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

/// Create a new cons-list.
///
/// Typically, it's easier to use the [cons!](macro.cons.html) macro for cons-list creation:
/// `cons![8, "Hi", 4.3]` is equivalent to `cons(8, cons("Hi", cons(4.3, Nil)))`.
///
/// # Example
///
/// ```
/// use lhlist::{cons, Cons, Nil};
/// let list = cons(8, cons("Hello!!!", cons(4.3, Nil)));
/// assert_eq!(list, cons![8, "Hello!!!", 4.3]);
/// assert_eq!(
///     list,
///     Cons {
///         head: 8,
///         tail: Cons {
///             head: "Hello!!!",
///             tail: Cons {
///                 head: 4.3,
///                 tail: Nil
///             }
///         }
///     }
/// );
/// ```
pub fn cons<H, T>(head: H, tail: T) -> Cons<H, T> {
    Cons { head, tail }
}

impl<Head, Tail> Cons<Head, Tail> {
    /// Returns a iterator over this cons-list.
    pub fn iter<'a>(&'a self) -> ConsIterator<'a, Self> {
        ConsIterator::new(self)
    }
}

/// A cons-list containing a set of labeled values.
pub type LVCons<Label, Tail> = Cons<LabeledValue<Label>, Tail>;
/// A cons-list containing only labels.
pub type LCons<Label, Tail> = Cons<PhantomData<Label>, Tail>;

impl Default for Nil {
    fn default() -> Nil { Nil }
}
impl<Lbl, Tail> Default for LCons<Lbl, Tail>
where
    Tail: Default
{
    fn default() -> LCons<Lbl, Tail> {
        Cons {
            head: PhantomData,
            tail: Tail::default()
        }
    }
}

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
    ///
    /// See [ValuesIterator](iter/struct.ValuesIterator.html) for an example.
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
    ///
    /// See [ConsIterator](iter/struct.ConsIterator.html) for an example.
    pub fn iter<'a>(&'a self) -> ConsIterator<'a, Self> {
        ConsIterator::new(self)
    }
    /// Returns an empty [ValuesIterator](iter/struct.ValuesIterator.html).
    pub fn iter_values<'a>(&'a self) -> ValuesIterator<'a, Self> {
        ValuesIterator::new(self)
    }
}

/// Macro for creation a [Cons](struct.Cons.html)-list.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate lhlist;
///
/// # fn main() {
/// let list = cons![8, "Hello!", 4.5];
///
/// let iter = list.iter();
/// let (item, iter) = iter.next();
/// assert_eq!(item, &8);
/// let (item, iter) = iter.next();
/// assert_eq!(item, &"Hello!");
/// let (item, _) = iter.next();
/// assert_eq!(item, &4.5);
/// # }
/// ```
#[macro_export]
macro_rules! cons {
    () => ( $crate::Nil );
    ($value:expr) => (
        $crate::cons($value, $crate::Nil)
    );
    ($value:expr, $($rest:tt)*) => (
        $crate::cons($value, cons![$($rest)*])
    );
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn cons_macro() {
        let list = cons![];
        assert_eq![list, Nil];

        let list = cons![8usize,];
        assert_eq![list, cons(8usize, Nil)];

        let list = cons![8usize, "Hello!!!"];
        assert_eq![list, cons(8usize, cons("Hello!!!", Nil))];

        let list = cons![8usize, "Hello!!!", 5.3];
        assert_eq![list, cons(8usize, cons("Hello!!!", cons(5.3, Nil)))];
        assert_eq![list,
            Cons {
                head: 8usize,
                tail: Cons {
                    head: "Hello!!!",
                    tail: Cons {
                        head: 5.3,
                        tail: Nil
                    }
                }
            }];
    }
}