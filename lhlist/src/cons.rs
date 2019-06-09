use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use crate::iter::{ConsIterator, ValuesIterator};
use crate::label::{LabeledValue, Value};
use crate::lookup::{LookupElemByLabel, LookupElemByLabelMut};
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

/// A cons-list containing a set of labeled values.
pub type LVCons<Label, Tail> = Cons<LabeledValue<Label>, Tail>;
/// A cons-list containing only labels.
pub type LCons<Label, Tail> = Cons<PhantomData<Label>, Tail>;

impl Default for Nil {
    fn default() -> Nil {
        Nil
    }
}
impl<Lbl, Tail> Default for LCons<Lbl, Tail>
where
    Tail: Default,
{
    fn default() -> LCons<Lbl, Tail> {
        Cons {
            head: PhantomData,
            tail: Tail::default(),
        }
    }
}

impl<Head, Tail> Cons<Head, Tail> {
    /// Returns a iterator over this cons-list.
    pub fn iter<'a>(&'a self) -> ConsIterator<'a, Self> {
        ConsIterator::new(self)
    }

    /// Returns `true` if target label exists in this list.
    ///
    /// Convenience function for calling
    /// [has_label_typearg](struct.Cons.html#method.has_label_typearg)
    /// without needing to specify type argument. Calling `list.has_label(ExampleLabel)` is
    /// equivalent to `list.has_label_typearg::<ExampleLabel>())`.
    pub fn has_label<TargetL>(&self, _target_label: TargetL) -> bool
    where
        Self: Member<TargetL>,
    {
        self.has_label_typearg::<TargetL>()
    }
    /// Returns `true` if target label exists in this list.
    ///
    /// Calling `list.has_label(ExampleLabel)` is equivalent to
    /// `list.has_label_typearg::<ExampleLabel>())`.
    pub fn has_label_typearg<TargetL>(&self) -> bool
    where
        Self: Member<TargetL>,
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

    /// Returns a reference the element labeled by a specific label.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate lhlist;
    /// use lhlist::labeled;
    /// # fn main() {
    /// new_label![Label1: u8];
    /// new_label![Label2: i8];
    /// new_label![Label3: bool];
    /// let list = lhlist![
    ///     Label1 = 9,
    ///     Label2 = -4,
    ///     Label3 = true,
    /// ];
    /// // assert_eq!(list[Label1], labeled(Label1, 9));
    /// assert_eq!(list.elem::<Label1>(), &labeled(Label1, 9));
    /// // assert_eq!(list[Label2], labeled(Label2, -4));
    /// assert_eq!(list.elem::<Label2>(), &labeled(Label2, -4));
    /// // assert_eq!(list[Label3], labeled(Label3, true));
    /// assert_eq!(list.elem::<Label3>(), &labeled(Label3, true));
    /// # }
    /// ```
    pub fn elem<TargetL>(&self) -> &<Self as LookupElemByLabel<TargetL>>::Elem
    where
        Self: LookupElemByLabel<TargetL>,
    {
        LookupElemByLabel::<TargetL>::elem(self)
    }

    /// Returns a mutable reference the element labeled by a specific label.
    ///
    /// This is equivalent to using index `list[Label]` notation in a mutable context.
    ///
    /// Note that the label itself cannot be changed via this method since labels in `lhlist` are
    /// type-level.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate lhlist;
    /// use lhlist::labeled;
    /// # fn main() {
    /// new_label![Label1: u8];
    /// new_label![Label2: i8];
    /// new_label![Label3: bool];
    /// let mut list = lhlist![
    ///     Label1 = 9,
    ///     Label2 = -4,
    ///     Label3 = true,
    /// ];
    ///
    /// let value2 = list.elem_mut::<Label2>();
    /// assert_eq!(value2, &mut labeled(Label2, -4));
    /// *value2 = labeled(Label2, -9);
    /// assert_eq!(list, lhlist![
    ///     Label1 = 9,
    ///     Label2 = -9,
    ///     Label3 = true,
    /// ]);
    /// # }
    /// ```
    pub fn elem_mut<TargetL>(&mut self) -> &mut <Self as LookupElemByLabel<TargetL>>::Elem
    where
        Self: LookupElemByLabelMut<TargetL>,
    {
        LookupElemByLabelMut::<TargetL>::elem_mut(self)
    }

    /// Returns a reference the value (e.g. the value portion of a
    /// [LabeledValue](struct.LabeledValue.html)) labeled by a specific label.
    ///
    /// This is equivalent to using index `list[Label]` notation, with the exception that `value`
    /// also works for transient (non-`'static`) types.
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # #[macro_use] extern crate lhlist;
    /// # fn main() {
    /// new_label![Label1: u8];
    /// new_label![Label2: i8];
    /// new_label![Label3: bool];
    /// let list = lhlist![
    ///     Label1 = 9,
    ///     Label2 = -4,
    ///     Label3 = true,
    /// ];
    /// assert_eq!(list[Label1], 9);
    /// assert_eq!(list.value::<Label1>(), &9);
    /// assert_eq!(list[Label2], -4);
    /// assert_eq!(list.value::<Label2>(), &-4);
    /// assert_eq!(list[Label3], true);
    /// assert_eq!(list.value::<Label3>(), &true);
    /// # }
    /// ```
    pub fn value<'a, TargetL>(
        &'a self,
    ) -> &'a <<Self as LookupElemByLabel<TargetL>>::Elem as Value>::Output
    where
        Self: LookupElemByLabel<TargetL>,
        <Self as LookupElemByLabel<TargetL>>::Elem: 'a + Value,
    {
        LookupElemByLabel::<TargetL>::elem(self).value_ref()
    }

    /// Returns a mutable reference the value (e.g. the value portion of a
    /// [LabeledValue](struct.LabeledValue.html)) labeled by a specific label.
    ///
    /// This is equivalent to using index `list[Label]` notation in `mut` contexts, with the
    /// exception that `value_mut` also works for transient (non-`'static`) types.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate lhlist;
    /// # fn main() {
    /// new_label![Label1: u8];
    /// new_label![Label2: i8];
    /// new_label![Label3: bool];
    /// let mut list = lhlist![
    ///     Label1 = 9,
    ///     Label2 = -4,
    ///     Label3 = true,
    /// ];
    ///
    /// let value2 = list.value_mut::<Label2>();
    /// assert_eq!(value2, &mut -4);
    /// *value2 = -9;
    /// assert_eq!(list, lhlist![
    ///     Label1 = 9,
    ///     Label2 = -9,
    ///     Label3 = true,
    /// ]);
    ///
    /// list[Label3] = false;
    /// assert_eq!(list, lhlist![
    ///     Label1 = 9,
    ///     Label2 = -9,
    ///     Label3 = false,
    /// ]);
    /// # }
    /// ```
    pub fn value_mut<'a, TargetL>(
        &'a mut self,
    ) -> &'a mut <<Self as LookupElemByLabel<TargetL>>::Elem as Value>::Output
    where
        Self: LookupElemByLabelMut<TargetL>,
        <Self as LookupElemByLabel<TargetL>>::Elem: 'a + Value,
    {
        LookupElemByLabelMut::<TargetL>::elem_mut(self).value_mut()
    }
}

impl Nil {
    /// Returns `false`, since the `Nil` list has no labels. See
    /// [has_label](type.LVCons.html#method.has_label) for more details.
    pub fn has_label<TargetL>(&self, _target_label: TargetL) -> bool {
        false
    }
    /// Returns `false`, since the `Nil` list has no labels. See
    /// [has_label_typearg](type.LVCons.html#method.has_label_typearg) for more details.
    pub fn has_label_typearg<TargetL>(&self) -> bool {
        false
    }

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

impl<L, H, T> Index<L> for Cons<H, T>
where
    Cons<H, T>: LookupElemByLabel<L>,
    <Cons<H, T> as LookupElemByLabel<L>>::Elem: 'static + Value,
{
    type Output = <<Cons<H, T> as LookupElemByLabel<L>>::Elem as Value>::Output;

    fn index(&self, _index: L) -> &Self::Output {
        LookupElemByLabel::<L>::elem(self).value_ref()
    }
}

impl<L, H, T> IndexMut<L> for Cons<H, T>
where
    Cons<H, T>: LookupElemByLabelMut<L>,
    <Cons<H, T> as LookupElemByLabel<L>>::Elem: 'static + Value,
{
    fn index_mut(&mut self, _index: L) -> &mut Self::Output {
        LookupElemByLabelMut::<L>::elem_mut(self).value_mut()
    }
}

/// Provides the length of a cons-list.
///
/// Since cons-list types are statically defined, this length is known at compile-time.
///
/// ## Example
///
/// ```
/// # #[macro_use] extern crate lhlist;
/// # fn main() {
/// use lhlist::{Cons, Len, Nil};
///
/// type MyList = Cons<usize, Cons<&'static str, Cons<f32, Nil>>>;
/// assert_eq!(MyList::LEN, 3usize);
///
/// let list: MyList = cons![8, "Hello!", 4.5];
/// assert_eq!(list.len(), 3usize);
/// # }
/// ```
pub trait Len {
    /// The length of this list
    const LEN: usize;

    /// Returns the length of this list
    fn len(&self) -> usize {
        Self::LEN
    }
}

impl Len for Nil {
    const LEN: usize = 0;
}
impl<H, T> Len for Cons<H, T>
where
    T: Len,
{
    const LEN: usize = 1 + <T as Len>::LEN;
}

/// Macro for creation of a [Cons](struct.Cons.html)-list.
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
        assert_eq![
            list,
            Cons {
                head: 8usize,
                tail: Cons {
                    head: "Hello!!!",
                    tail: Cons {
                        head: 5.3,
                        tail: Nil
                    }
                }
            }
        ];
    }
}
