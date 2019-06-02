use typenum::Unsigned;

/// A trait with information about a label.
///
/// Typically, labels are simply unit-like structs used to identify elements in a list. This trait
/// contains the label's name, an associated type, and an internal identifier.
///
/// It is encouraged that this trait be implemented using `#[label]`, which ensures that the
/// identifier `Uid` is unique.
///
/// ## Examples
/// Basic label creation. The label's name is set to the identifier name.
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
/// You can provide a custom name by passing `name="Label Name"`.
/// ```
/// # #[macro_use] extern crate lhlist;
///
/// # fn main() {
/// use lhlist::Label;
///
/// #[label(name="My Fantastic Label")]
/// struct MyLabel;
///
/// assert_eq!(MyLabel::name(), "My Fantastic Label");
/// # }
/// ```
///
/// You can provide the associated type (which otherwise is defaulted to `()`) by using either
/// `type=<type>` or `assoc_type=<type>`.
/// ```
/// # #[macro_use] extern crate lhlist;
///
/// # fn main() {
/// use lhlist::Label;
///
/// #[label(name="My Amazing Label", assoc_type=u8)]
/// struct MyLabel;
///
/// assert_eq!(MyLabel::name(), "My Amazing Label");
/// assert_eq!(<MyLabel as Label>::AssocType::max_value(), u8::max_value());
///
/// #[label(type=u32)]
/// struct MyOtherLabel;
///
/// assert_eq!(MyOtherLabel::name(), "MyOtherLabel");
/// assert_eq!(<MyOtherLabel as Label>::AssocType::max_value(), u32::max_value());
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

impl<L> Label for std::marker::PhantomData<L> where L: Label {
    const NAME: &'static str = L::NAME;
    type AssocType = L::AssocType;
    type Uid = L::Uid;
}

/// A value along with its label.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Labeled<L: Label> {
    /// Labeled value
    pub value: L::AssocType,
}
impl<L> Labeled<L> where L: Label {
    /// Create a new labeled value.
    pub fn new(value: L::AssocType) -> Labeled<L> {
        Labeled { value }
    }
}
impl<L> Label for Labeled<L> where L: Label {
    const NAME: &'static str = L::NAME;
    type AssocType = L::AssocType;
    type Uid = L::Uid;
}
/// Creates a new [Labeled](struct.Labeled.html) object for placement into an
/// [LVCons](type.LVCons.html) list.
pub fn new_labeled<L>(_label: L) -> Labeled<L> where L: Label<AssocType=()> {
    new_labeled_typearg::<L>()
}
/// Creates a new [Labeled](struct.Labeled.html) object for placement into an
/// [LVCons](type.LVCons.html) list.
pub fn new_labeled_typearg<L>() -> Labeled<L> where L: Label<AssocType=()> {
    Labeled { value: () }
}

/// Macro for creating [LCons](type.LCons.html) label-only cons-lists.
#[macro_export]
macro_rules! labels {
    () => ( Nil );
    ($label:ty) => (
        Cons {
            head: std::marker::PhantomData::<$label>,
            tail: Nil
        }
    );
    ($label:ty, $($rest:tt)*) => (
        Cons {
            head: std::marker::PhantomData::<$label>,
            tail: labels![$($rest)*]
        }
    );
}

/// Macro for creating [LVCons](type.LVCons.html) label-value cons-lists.
#[macro_export]
macro_rules! lhlist {
    () => ( $crate::Nil );
    ($label:ty = $value:expr) => (
        $crate::Cons {
            head: $crate::Labeled::<$label>::new($value),
            tail: Nil
        }
    );
    ($label:ty = $value:expr, $($rest:tt)*) => (
        $crate::Cons {
            head: $crate::Labeled::<$label>::new($value),
            tail: lhlist![$($rest)*]
        }
    );
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[label(name = "My Label")]
    struct Label1;

    #[label(type=u8)]
    struct Label2;

    #[label]
    struct Label3;

    #[test]
    fn label_create() {
        assert_eq!(Label1::name(), "My Label");

        assert_eq!(Label2::name(), "Label2");
        assert_eq!(<Label2 as Label>::AssocType::max_value(), 255u8);

        assert_eq!(Label3::name(), "Label3");
    }
}
