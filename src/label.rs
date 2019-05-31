use std::marker::PhantomData;

use typenum::Unsigned;

/// A trait with information about a label.
///
/// Contains the label's name and internal identifier.
///
/// It is encouraged that this trait be derived using `#[derive(Label)]`, which ensures that the
/// identifier `Uid` is unique. Typically, labels are simply unit-like structs used to identify
/// elements in a list.
///
/// ## Examples
/// Basic label creation (name is set to variable name):
/// ```
/// # #[macro_use] extern crate lhlist;
///
/// # fn main() {
/// use lhlist::Label;
///
/// #[label]
/// struct MyLabel;
///
/// assert_eq!(MyLabel::name(), "MyLabel");
/// # }
/// ```
///
/// Custom name:
/// ```
/// # #[macro_use] extern crate lhlist;
///
/// # fn main() {
/// use lhlist::Label;
///
/// #[label(name = "My Fantastic Label", assoc_type = u64)]
/// struct MyLabel;
///
/// assert_eq!(MyLabel::name(), "My Fantastic Label");
/// # }
/// ```
pub trait Label {
    /// Name of this label (for display /output)
    const NAME: &'static str;
    /// Associated type of this label
    type AssocType;
    /// Internal unique label identifier
    type Uid: Unsigned;

    /// Returns the label's name (by default, variable name)
    fn name() -> &'static str {
        Self::NAME
    }
    /// Returns the label's unique identifier
    fn id() -> usize {
        Self::Uid::to_usize()
    }
}

/// A value along with its label.
#[derive(Debug, Clone)]
pub struct Labeled<L, V> {
    _label: PhantomData<L>,
    /// Labeled value
    pub value: V,
}
impl<L, V> From<V> for Labeled<L, V> {
    fn from(orig: V) -> Labeled<L, V> {
        Labeled {
            _label: PhantomData,
            value: orig
        }
    }
}
impl<L, V> Label for Labeled<L, V> where L: Label {
    const NAME: &'static str = L::NAME;
    type AssocType = L::AssocType;
    type Uid = L::Uid;
}
/// Creates a new [Labeled](struct.Labeled.html) object for placement into an
/// [LVCons](type.LVCons.html) list.
pub fn new_labeled<L>(_label: L) -> Labeled<L, ()> {
    Labeled::<L, ()>::from(())
}
/// Creates a new [Labeled](struct.Labeled.html) object for placement into an
/// [LVCons](type.LVCons.html) list.
pub fn new_labeled_typearg<L>() -> Labeled<L, ()> {
    Labeled::<L, ()>::from(())
}

/// Macro for creating [LCons](type.LCons.html) label-only cons-lists.
#[macro_export]
macro_rules! lcons {
    () => ( Nil );
    ($label:ty) => (
        Cons {
            head: new_labeled_typearg::<$label>(),
            tail: Nil
        }
    );
    ($label:ty, $($rest:tt)*) => (
        Cons {
            head: new_labeled_typearg::<$label>(),
            tail: lcons![$($rest)*]
        }
    );
}

/// Macro for creating [LVCons](type.LVCons.html) label-value cons-lists.
#[macro_export]
macro_rules! lvcons {
    () => ( Nil );
    ($label:ty = $value:expr) => (
        Cons {
            head: Labeled::<$label, _>::from($value),
            tail: Nil
        }
    );
    ($label:ty = $value:expr, $($rest:tt)*) => (
        Cons {
            head: Labeled::<$label, _>::from($value),
            tail: lvcons![$($rest)*]
        }
    );
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
    fn label_create() {
        assert_eq!(Label1::name(), "My Label");

        assert_eq!(Label2::name(), "Label2");
        assert_eq!(<Label2 as Label>::AssocType::max_value(), 255u8);

        assert_eq!(Label3::name(), "Label3");
    }
}