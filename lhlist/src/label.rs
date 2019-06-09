use typenum::Unsigned;

use crate::cons::{Cons, LCons, LVCons, Len, Nil};

/// A trait with information about a label.
///
/// Typically, labels are simply unit-like structs used to identify elements in a list. This trait
/// contains the label's name, an associated type, and an internal identifier.
///
/// It is encouraged that this trait be implemented using the [new_label](macro.new_label.html)
/// macro or the `#[label]` attribute, which ensures that the identifier `Uid` is unique. See the
/// documentation for [new_label](macro.new_label.html) for examples.
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

impl<L> Label for std::marker::PhantomData<L>
where
    L: Label,
{
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
impl<L> LabeledValue<L>
where
    L: Label,
{
    /// Create a new labeled value.
    pub fn new(value: L::AssocType) -> LabeledValue<L> {
        LabeledValue { value }
    }
}
impl<L> Label for LabeledValue<L>
where
    L: Label,
{
    const NAME: &'static str = L::NAME;
    type AssocType = L::AssocType;
    type Uid = L::Uid;
}
/// Creates a new [LabeledValue](struct.LabeledValue.html) object for placement into an
/// [LVCons](type.LVCons.html) list.
pub fn labeled<L>(_label: L, value: L::AssocType) -> LabeledValue<L>
where
    L: Label,
{
    labeled_typearg::<L>(value)
}
/// Creates a new [LabeledValue](struct.LabeledValue.html) object for placement into an
/// [LVCons](type.LVCons.html) list.
pub fn labeled_typearg<L>(value: L::AssocType) -> LabeledValue<L>
where
    L: Label,
{
    LabeledValue { value }
}

/// A trait that provides access to contained 'values'.
pub trait Value {
    /// The type of the contained value.
    type Output: ?Sized;
    /// Immutable reference to the contained value
    fn value_ref(&self) -> &Self::Output;
    /// Mutable reference to the contained value
    fn value_mut(&mut self) -> &mut Self::Output;
}
impl<L> Value for LabeledValue<L>
where
    L: Label,
{
    type Output = L::AssocType;
    fn value_ref(&self) -> &Self::Output {
        &self.value
    }
    fn value_mut(&mut self) -> &mut Self::Output {
        &mut self.value
    }
}

/// Trait for extracting the labels ([LCons](type.LCons.html)) from a cons-list of elements which
/// all implement [Label](trait.Label.html).
pub trait HasLabels {
    /// Associated labels
    type Labels;

    /// Instantiates a cons-list only containing label information.
    fn labels_only(&self) -> Self::Labels
    where
        Self::Labels: Default,
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

/// Generate a `Vec` containing the names of the labels in a labeled cons-list.
///
/// ## Example
/// ```
/// # #[macro_use] extern crate lhlist;
/// use lhlist::StrLabels;
/// # fn main() {
/// new_label![MyLabel1];
/// new_label![MyLabel2];
/// new_label![MyLabel3];
///
/// let list = lhlist![MyLabel1 = (), MyLabel2 = (), MyLabel3 = ()];
/// assert_eq!(list.labels(), vec!["MyLabel1", "MyLabel2", "MyLabel3"]);
/// # }
/// ```
pub trait StrLabels {
    /// Generates the label name `Vec`
    fn static_labels() -> Vec<&'static str>;
    /// Generates the label name `Vec` using a value
    fn labels(&self) -> Vec<&'static str> {
        Self::static_labels()
    }
}
impl StrLabels for Nil {
    fn static_labels() -> Vec<&'static str> {
        vec![]
    }
}
impl<Lbl: Sized, Tail> StrLabels for Cons<Lbl, Tail>
where
    Self: Len + BuildStrLabels,
{
    fn static_labels() -> Vec<&'static str> {
        let mut output = vec![""; Self::LEN];
        Self::build_labels(&mut output, 0);
        output
    }
}

pub trait BuildStrLabels {
    fn build_labels(v: &mut Vec<&'static str>, idx: usize);
}
impl BuildStrLabels for Nil {
    fn build_labels(v: &mut Vec<&'static str>, idx: usize) {
        debug_assert![idx == v.len()];
    }
}
impl<Lbl, Tail> BuildStrLabels for Cons<Lbl, Tail>
where
    Lbl: Label,
    Tail: BuildStrLabels,
{
    fn build_labels(v: &mut Vec<&'static str>, idx: usize) {
        debug_assert![idx < v.len()];
        v[idx] = Lbl::NAME;
        Tail::build_labels(v, idx + 1);
    }
}

/// Macro for easily creating a label struct.
///
/// There are three formats for calling this macro:
/// ```
/// # #[macro_use] extern crate lhlist;
/// # fn main() {
/// use lhlist::Label;
///
/// // default name (variable name), nil associated type `()`
/// new_label![MyLabel1];
/// assert_eq!(MyLabel1::name(), "MyLabel1");
///
/// // default name (variable name) with associated type
/// new_label![MyLabel2: u8];
/// assert_eq!(MyLabel2::name(), "MyLabel2");
/// assert_eq!(<MyLabel2 as Label>::AssocType::max_value(), u8::max_value());
///
/// new_label![MyLabel3("My Custom Label"): u16];
/// assert_eq!(MyLabel3::name(), "My Custom Label");
/// assert_eq!(<MyLabel3 as Label>::AssocType::max_value(), u16::max_value());
/// # }
/// ```
///
/// Alternatively (and equivalently), you can use the `#[label(type=T, name="Name")]`
/// attribute format:
/// ```
/// # #[macro_use] extern crate lhlist;
/// # fn main() {
/// use lhlist::Label;
/// #[label]
/// struct MyLabel1;
/// assert_eq!(MyLabel1::name(), "MyLabel1");
///
/// #[label(type=Vec<u32>)]
/// struct MyLabel2;
/// assert_eq!(MyLabel2::name(), "MyLabel2");
///
/// #[label(name="My Custom Label", type=Vec<u32>)]
/// struct MyLabel3;
/// assert_eq!(MyLabel3::name(), "My Custom Label");
/// # }
/// ```
#[macro_export]
macro_rules! new_label {
    ($id:ident) => {
        #[label]
        struct $id;
    };
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

/// Macro for creating labeled heterogeneous lists.
///
/// Create an [LVCons](type.LVCons.html) label-value cons-list based on a comma-separated list
/// of `<label> = <value>` assignments.
///
/// # Example
/// ```
/// # #[macro_use] extern crate lhlist;
/// # fn main() {
/// new_label![Name: &'static str];
/// new_label![Planets: Vec<&'static str>];
/// new_label![NumDwarfPlanets: usize]; // currently recognized
/// new_label![Age: f32]; // in billions of years
/// new_label![Mass: f32]; // in Solar masses
///
/// let solar_system = lhlist![
///     Name = "Solar System",
///     Planets = vec![
///         "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune"
///     ],
///     NumDwarfPlanets = 5,
///     Age = 4.568,
///     Mass = 1.0014,
/// ];
/// assert_eq!(solar_system[Planets][2], "Earth"); // third rock from the sun
/// assert_eq!(solar_system[Age], 4.568);
/// # }
/// ```
#[macro_export]
macro_rules! lhlist {
    () => ( $crate::Nil );
    ($label:ty = $value:expr) => (
        $crate::Cons {
            head: $crate::LabeledValue::<$label>::new($value),
            tail: $crate::Nil
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
    use crate::*;
    use std::marker::PhantomData;

    #[test]
    fn label_create() {
        #[label(name = "My Label", crate=crate)]
        struct Label1;
        #[label(type=u8, crate=crate)]
        struct Label2;
        #[label(crate=crate)]
        struct Label3;

        assert_eq!(Label1::name(), "My Label");

        assert_eq!(Label2::name(), "Label2");
        assert_eq!(<Label2 as Label>::AssocType::max_value(), 255u8);

        assert_eq!(Label3::name(), "Label3");
    }

    #[test]
    fn has_labels() {
        #[label(type=u8, crate=crate)]
        struct Label1;
        #[label(type=u16, crate=crate)]
        struct Label2;
        #[label(type=i16, crate=crate)]
        struct Label3;
        let test_list = lhlist![Label1 = 2, Label2 = 301, Label3 = -523,];
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
