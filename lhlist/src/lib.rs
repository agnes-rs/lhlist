/*!
Library for labeled heterogeneous lists.

This crate provides data structures and traits for assembling and using labeled heterogeneous lists.
A heterogeneous list is a list of values where each value has a different types. These heterogeneous
lists are implemented using [cons-lists](https://en.wikipedia.org/wiki/Cons#Lists), which adds some
usage flexibility over what is available with built-in Rust tuples.

## Labels

Lists created in `lhlist` possess 'labels' used to identify and index into the list. Labels are
unit-like structs which implement the [Label](label/trait.Label.html) trait and are typically
created using the [new_label](macro.new_label.html) macro.

```
# #[macro_use] extern crate lhlist;
# fn main() {
use lhlist::Label;

new_label![MyLabel: u8];
assert_eq!(MyLabel::name(), "MyLabel");
# }
```

See the [new_label](macro.new_label.html) documentation for more examples of creating labels.

## List Creation

Labeled lists are created using the [lhlist](macro.lhlist.html) macro, which takes a
comma-separated list of label-value pairs. For example,

```
# #[macro_use] extern crate lhlist;
# fn main() {
use lhlist::Label;

new_label![SomeNumbers: Vec<u64>];
new_label![SomeNames: Vec<&'static str>];
new_label![Flag: bool];

let my_list = lhlist![
    SomeNumbers = vec![0, 4, 5, 2],
    SomeNames = vec!["hello", "world!"],
    Flag = false,
];
assert_eq!(my_list[Flag], false);
# }
```

Internally, the values are contained as [LabeledValue](struct.LabeledValue.html) structs which
associate the label information to the added value.

## Accessing via Label

The [LabeledValue](struct.LabeledValue.html) objects contained in a list can be accessed via
the [elem](struct.Cons.html#method.elem) or [elem_mut](struct.Cons.html#method.elem_mut) methods.

The contained values in a list (without the associated label information) can be accessed via
the [value](struct.Cons.html#method.value) or [value_mut](struct.Cons.html#method.value_mut)
methods. Using `list[Label]` notation (via `Index` and `IndexMut`) is also supported.

More details and examples can be found in the documentation for the various accessor methods.

## Iteration

Much like accessing individual element of a list, iteration over a list can be done in two contexts:
1. Calling [iter](struct.Cons.html#method.iter) to create a
[ConsIterator](iter/struct.ConsIterator.html) which iterates over the
[LabeledValue](struct.LabeledValue.html) objects, or
2. Calling [iter_values](struct.Cons.html#method.iter_values) to create a
[ValuesIterator](iter/struct.ValuesIterator.html) which iterates over the contained values.

These iterators both support [mapping](iter/struct.MapAdapter.html) functionality for processing
values using types that implement the [MapFunc](iter/trait.MapFunc.html) trait. See the
[MapAdapter](iter/struct.MapAdapter.html) documentation for more details and an example.

There are also two ways to collect the contents of an iterator into a new cons-list:
1. [CollectIntoHList](iter/trait.CollectIntoHList.html) collects the contents of an iterator as-is.
When this collection is performed on a [ValuesIterator](iter/struct.ValuesIterator.html), the new
cons-list does not contain any label information (since the `ValueIterator` only iterates over the
contained values).
2. [CollectIntoLabeledHList](iter/trait.CollectIntoLabeledHList.html) collects the contents of an
iterator with a new provided set of labels. This is particularly useful when the types of elements
have changed during the iterator process (via [MapAdapter](iter/struct.MapAdapter.html)) and the
old labels are no longer valid (since a label can only has one associated type).

An example of both kinds of collection can be see in the [MapAdapter](iter/struct.MapAdapter.html)
documentation.
*/

#![warn(missing_docs)]

extern crate label_attribute;
extern crate typenum;

pub use label_attribute::*;

mod cons;
pub use cons::{cons, Cons, LCons, LVCons, Len, Nil};

mod label;
pub use label::{labeled, labeled_typearg, HasLabels, Label, LabeledValue, StrLabels, Value};

mod relation;
pub use relation::{Bool, False, LabelEq, Member, ToBool, True};

mod lookup;
pub use lookup::LookupElemByLabel;

mod ordered_set;

pub mod iter;
