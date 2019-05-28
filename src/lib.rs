/*!
Library for labeled heterogeneous lists.




*/

#![warn(missing_docs)]

extern crate typenum;
extern crate label_derive;

pub use label_derive::*;


mod cons;
pub use cons::{Nil, Cons, LCons, LVCons};

mod label;
pub use label::{Label, Labeled, new_labeled, new_labeled_typearg};

mod relation;
pub use relation::{True, False, Bool, ToBool, LabelEq, Member};

mod lookup;
pub use lookup::{LookupElemByLabel};

pub mod iter;

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn label_eq() {
        assert!(<Label1 as LabelEq<Label1>>::Output::VALUE);
        assert!(<Label2 as LabelEq<Label2>>::Output::VALUE);
        assert!(<Label3 as LabelEq<Label3>>::Output::VALUE);

        assert!(!<Label1 as LabelEq<Label2>>::Output::VALUE);
        assert!(!<Label1 as LabelEq<Label3>>::Output::VALUE);

        assert!(!<Label2 as LabelEq<Label1>>::Output::VALUE);
        assert!(!<Label2 as LabelEq<Label3>>::Output::VALUE);

        assert!(!<Label3 as LabelEq<Label1>>::Output::VALUE);
        assert!(!<Label3 as LabelEq<Label2>>::Output::VALUE);
    }

    #[test]
    fn member() {
        // type-based member testing
        type TestList = LCons<Label1, LCons<Label2, Nil>>;
        assert!(<TestList as Member<Label1>>::Output::VALUE);
        assert!(<TestList as Member<Label2>>::Output::VALUE);
        assert!(!<TestList as Member<Label3>>::Output::VALUE);

        // value-based member testing
        let list = lcons![Label1, Label2];
        assert!(list.has_label(Label1));
        assert!(list.has_label(Label2));
        assert!(!list.has_label(Label3));
    }

    #[test]
    fn lookup() {
        let list = lcons![Label1, Label2];
        println!("{:?}", LookupElemByLabel::<Label2>::elem(&list));
    }
}
