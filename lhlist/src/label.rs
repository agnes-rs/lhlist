use typenum::Unsigned;

use crate::cons::{Nil, LCons, LVCons};

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
pub struct LabeledValue<L: Label> {
    /// LabeledValue value
    pub value: L::AssocType,
}
impl<L> LabeledValue<L> where L: Label {
    /// Create a new labeled value.
    pub fn new(value: L::AssocType) -> LabeledValue<L> {
        LabeledValue { value }
    }
}
impl<L> Label for LabeledValue<L> where L: Label {
    const NAME: &'static str = L::NAME;
    type AssocType = L::AssocType;
    type Uid = L::Uid;
}
/// Creates a new [LabeledValue](struct.LabeledValue.html) object for placement into an
/// [LVCons](type.LVCons.html) list.
pub fn labeled<L>(_label: L, value: L::AssocType) -> LabeledValue<L> where L: Label {
    labeled_typearg::<L>(value)
}
/// Creates a new [LabeledValue](struct.LabeledValue.html) object for placement into an
/// [LVCons](type.LVCons.html) list.
pub fn labeled_typearg<L>(value: L::AssocType) -> LabeledValue<L> where L: Label {
    LabeledValue { value }
}

/// Trait for extracting the labels ([LCons](type.LCons.html)) from a cons-list of elements which
/// all implement [Label](trait.Label.html).
pub trait HasLabels {
    /// Associated labels
    type Labels;

    /// Instantiates a cons-list only containing label information.
    fn labels_only(&self) -> Self::Labels
    where
        Self::Labels: Default
    {
        Self::Labels::default()
    }
}

impl HasLabels for Nil {
    type Labels = Nil;
}
impl<Lbl, Tail> HasLabels for LVCons<Lbl, Tail>
where
    Lbl: Label,
    Tail: HasLabels,
{
    type Labels = LCons<Lbl, Tail::Labels>;
}

/// Macro for easily creating a label struct.
///
/// There are two formats for calling this macro:
/// ```
/// # #[macro_use] extern crate lhlist;
/// # fn main() {
/// use lhlist::Label;
/// new_label![MyLabel: Vec<u32>];
/// assert_eq!(MyLabel::name(), "MyLabel");
/// # }
/// ```
/// which provides a default name 'MyLabel' to the created label, and
/// ```
/// # #[macro_use] extern crate lhlist;
/// # fn main() {
/// use lhlist::Label;
/// new_label![MyLabel("My Very Own Label"): Vec<u32>];
/// assert_eq!(MyLabel::name(), "My Very Own Label");
/// # }
/// ```
/// which supplies an explicit name.
///
/// Alternatively (and equivalently), you can use the `#[label(type=T)]` attribute format:
/// ```
/// # #[macro_use] extern crate lhlist;
/// # fn main() {
/// use lhlist::Label;
///
/// #[label(type=Vec<u32>)]
/// struct MyLabel;
/// assert_eq!(MyLabel::name(), "MyLabel");
/// # }
/// ```
/// or
/// ```
/// # #[macro_use] extern crate lhlist;
/// # fn main() {
/// use lhlist::Label;
///
/// #[label(name="My Very Own Label", type=Vec<u32>)]
/// struct MyLabel;
/// assert_eq!(MyLabel::name(), "My Very Own Label");
/// # }
/// ```
#[macro_export]
macro_rules! new_label {
    ($id:ident: $type:ty) => {
        #[label(type=$type)]
        struct $id;
    };
    ($id:ident($name:expr): $type:ty) => {
        #[label(name=$name, type=$type)]
        struct $id;
    };
}

/// Macro for creating type signature for a [LCons](type.LCons.html) label-only cons-list.
///
/// This type signature can be useful for specifying a list of labels for methods and functions
/// that require them.
#[macro_export]
macro_rules! Labels {
    () => ( $crate::Nil );
    ($label:ty) => (
        $crate::LCons<$label, $crate::Nil>
    );
    ($label:ty, $($rest:tt)*) => (
        $crate::LCons<$label, Labels![$($rest)*]>
    )
}

/// Macro for creating an instance of an [LCons](type.LCons.html) label-only cons-lists.
#[macro_export]
macro_rules! labels {
    () => ( $crate::Nil );
    ($label:ty) => (
        $crate::Cons {
            head: std::marker::PhantomData::<$label>,
            tail: $crate::Nil
        }
    );
    ($label:ty, $($rest:tt)*) => (
        $crate::Cons {
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
            head: $crate::LabeledValue::<$label>::new($value),
            tail: Nil
        }
    );
    ($label:ty = $value:expr, $($rest:tt)*) => (
        $crate::Cons {
            head: $crate::LabeledValue::<$label>::new($value),
            tail: lhlist![$($rest)*]
        }
    );
}


#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use crate::*;


    #[test]
    fn label_create() {
        #[label(name = "My Label")]
        struct Label1;
        #[label(type=u8)]
        struct Label2;
        #[label]
        struct Label3;

        assert_eq!(Label1::name(), "My Label");

        assert_eq!(Label2::name(), "Label2");
        assert_eq!(<Label2 as Label>::AssocType::max_value(), 255u8);

        assert_eq!(Label3::name(), "Label3");
    }

    #[test]
    fn has_labels() {
        #[label(type=u8)]
        struct Label1;
        #[label(type=u16)]
        struct Label2;
        #[label(type=i16)]
        struct Label3;
        let test_list = lhlist![
            Label1 = 2,
            Label2 = 301,
            Label3 = -523,
        ];
        let iter = test_list.iter();
        let (item, iter) = iter.next();
        assert_eq!(item, &labeled(Label1, 2));
        let (item, iter) = iter.next();
        assert_eq!(item, &labeled(Label2, 301));
        let (item, _) = iter.next();
        assert_eq!(item, &labeled(Label3, -523));


        let labels_only = test_list.labels_only();
        let iter = labels_only.iter();
        let (item, iter) = iter.next();
        assert_eq!(item, &PhantomData::<Label1>::default());
        let (item, iter) = iter.next();
        assert_eq!(item, &PhantomData::<Label2>::default());
        let (item, _) = iter.next();
        assert_eq!(item, &PhantomData::<Label3>::default());
    }
}
