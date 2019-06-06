/*!
Library for labeled heterogeneous lists.

This crate provides data structures and traits for assembling and using labeled heterogeneous lists.
A heterogeneous list is a list of values where each value has a different types. These heterogeneous
lists are implemented using [cons-lists](https://en.wikipedia.org/wiki/Cons#Lists), which adds some
usage flexibility over what is available with built-in Rust tuples.

Lists created in `lhlist` posess 'labels' use to identify and index into the list. Labels are
unit-like structs which implement the [Label](label/trait.Label.html) trait and are typically
created using the `#[label]` attribute-like macro.

```
# #[macro_use] extern crate lhlist;
# fn main() {
use lhlist::Label;
#[label]
struct MyLabel;
assert_eq!(MyLabel::name(), "MyLabel");
# }
```

See the [Label](label/trait.Label.html) documentation for more examples of using `#[label]`.

Labeled lists are created using the `[lhlist](label/macro.lhlist.html)` macro, which takes a
comma-separated list of label-value pairs. For example,

```
# #[macro_use] extern crate lhlist;
# fn main() {
use lhlist::Label;

#[label(type=Vec<u64>)]
struct SomeNumbers;

#[label(type=Vec<&'static str>)]
struct SomeNames;

#[label(type=bool)]
struct Flag;

let my_list = lhlist![
    SomeNumbers = vec![0, 4, 5, 2],
    SomeNames = vec!["hello", "world!"],
    Flag = false,
];
# }
```

*/

#![warn(missing_docs)]

extern crate typenum;
extern crate label_attribute;

pub use label_attribute::*;


mod cons;
pub use cons::{Nil, cons, Cons, LCons, LVCons};

mod label;
pub use label::{Label, LabeledValue, labeled, labeled_typearg};

mod relation;
pub use relation::{True, False, Bool, ToBool, LabelEq, Member};

mod lookup;
pub use lookup::{LookupElemByLabel};

pub mod iter;
