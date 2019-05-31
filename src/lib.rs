/*!
Library for labeled heterogeneous lists.





*/

#![warn(missing_docs)]

extern crate typenum;
extern crate label_attribute;

pub use label_attribute::*;


mod cons;
pub use cons::{Nil, Cons, LCons, LVCons};

mod label;
pub use label::{Label, Labeled, new_labeled, new_labeled_typearg};

mod relation;
pub use relation::{True, False, Bool, ToBool, LabelEq, Member};

mod lookup;
pub use lookup::{LookupElemByLabel};

pub mod iter;
