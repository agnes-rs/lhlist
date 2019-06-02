/*!
Iteration over heterogeneous cons-lists and labeled heterogeneous cons-lists.


*/
use crate::cons::{Cons, LVCons, Nil};
use crate::label::{Label, Labeled};

/// An iterator over a heterogeneous cons-list ([Cons](../struct.Cons.html));
#[derive(Debug)]
pub struct ConsIterator<'a, List, A=Nil> {
    list: &'a List,
    adapter: A,
}

impl<'a, List> ConsIterator<'a, List> {
    /// Creates a new `ConsIterator` over an `Cons`-list
    pub fn new(list: &'a List) -> Self {
        ConsIterator { list, adapter: Nil }
    }
}
impl<'a, List, A> ConsIterator<'a, List, A> {
    /// Creates a new `ConsIterator` over an `Cons`-list with a specified adapter (see
    /// [Adapter](trait.Adapter.html)).
     pub fn with_adapter(list: &'a List, adapter: A) -> Self {
        ConsIterator {
            list,
            adapter
        }
    }
}

impl<'a, H, T, A> ConsIterator<'a, Cons<H, T>, A>
where
    A: Adapter<&'a H>,
{
    /// Returns the next value (if exists) along with a new iterator advanced to the next element of
    /// the list.
    pub fn next(mut self) -> (<A as Adapter<&'a H>>::Output, ConsIterator<'a, T, A>) {
        (
            self.adapter.adapt(&self.list.head),
            ConsIterator::with_adapter(&self.list.tail, self.adapter)
        )
    }
    /// Creates an iterator which call a [MapFunc](trait.MapFunc.html) on each element.
    pub fn map<F>(self, f: F) -> ConsIterator<'a, Cons<H, T>, Cons<MapAdapter<F>, A>>
    where
        F: MapFunc<<A as Adapter<&'a H>>::Output>
    {
        ConsIterator::with_adapter(self.list, Cons { head: MapAdapter { f }, tail: self.adapter })
    }
}


/// An iterator over a labeled heterogeneous cons-list ([LVCons](../type.LVCons.html)) that only
/// provides access to the values.
#[derive(Debug)]
pub struct ValuesIterator<'a, List, A=Nil> {
    list: &'a List,
    adapter: A,
}

impl<'a, List> ValuesIterator<'a, List> {
    /// Creates a new `ValuesIterator` over an `LVCons`-list
    pub fn new(list: &'a List) -> Self {
        ValuesIterator { list, adapter: Nil }
    }
}
impl<'a, List, A> ValuesIterator<'a, List, A> {
    /// Creates a new `ValuesIterator` over an `LVCons`-list with a specified adapter (see
    /// [Adapter](trait.Adapter.html)).
    pub fn with_adapter(list: &'a List, adapter: A) -> Self {
        ValuesIterator {
            list,
            adapter
        }
    }
}

impl<'a, L, T, A> ValuesIterator<'a, LVCons<L, T>, A>
where
    L: Label,
    A: Adapter<&'a L::AssocType>,
{
    /// Returns the next value (if exists) along with a new iterator advanced to the next element of
    /// the list.
    pub fn next(mut self)
        -> (<A as Adapter<&'a L::AssocType>>::Output, ValuesIterator<'a, T, A>)
    {
        (
            self.adapter.adapt(&self.list.head.value),
            ValuesIterator::with_adapter(&self.list.tail, self.adapter)
        )
    }
    /// Creates an iterator which call a [MapFunc](trait.MapFunc.html) on each element.
    pub fn map<F>(self, f: F) -> ValuesIterator<'a, LVCons<L, T>, Cons<MapAdapter<F>, A>>
    where
        F: MapFunc<<A as Adapter<&'a L::AssocType>>::Output>
    {
        ValuesIterator::with_adapter(self.list, Cons { head: MapAdapter { f }, tail: self.adapter })
    }
}




/// An iterator component that transforms an input.
pub trait Adapter<T> {
    /// Transformation output type
    type Output;
    /// Transforms the input and returns its output
    fn adapt(&mut self, input: T) -> Self::Output;
}

impl<T> Adapter<T> for Nil {
    type Output = T;
    fn adapt(&mut self, input: T) -> Self::Output { input }
}

impl<T, Head, Tail> Adapter<T> for Cons<Head, Tail>
where
    Tail: Adapter<T>,
    Head: Adapter<<Tail as Adapter<T>>::Output>,
{
    type Output = <Head as Adapter<<Tail as Adapter<T>>::Output>>::Output;

    fn adapt(&mut self, input: T) -> Self::Output {
        self.head.adapt(self.tail.adapt(input))
    }
}

/// Function mapping iterator component.
///
/// Transforms input using a function implementing [MapFunc](trait.MapFunc.html).
#[derive(Debug)]
pub struct MapAdapter<F> {
    f: F
}
impl<F, T> Adapter<T> for MapAdapter<F>
where
    F: MapFunc<T>
{
    type Output = <F as MapFunc<T>>::Output;
    fn adapt(&mut self, input: T) -> Self::Output { self.f.call(input) }
}

/// Function for use in mapping over heterogeneous lists.
///
/// This trait must be implemented for all types contained in the list.
pub trait MapFunc<T> {
    /// Output of mapped function
    type Output;
    /// Evaluate this function on the input
    fn call(&mut self, item: T) -> Self::Output;
}

/// Collects an iterator (either [ConsIterator](struct.ConsIterator.html) or
/// [ValuesIterator](struct.ValuesIterator.html)) into a hetereogeneous cons-list.
///
/// The resulting cons-list does not necessarily maintain label information -- it typically returns
/// a [Cons](../struct.Cons.html)-list instead of an [LVCons](../type.LVCons.html)-list. See
/// [CollectIntoLabeledHList](trait.CollectIntoLabeledHList.html) for a version that returns
/// a labeled cons-list if you wish to preserve label information.
pub trait CollectIntoHList {
    /// Output type of collected list
    type Output;
    /// Collects the contents of an iterator into a cons-list
    fn collect_into_hlist(self) -> Self::Output;
}

impl<'a, A> CollectIntoHList for ValuesIterator<'a, Nil, A> {
    type Output = Nil;
    fn collect_into_hlist(self) -> Self::Output { Nil }
}

impl<'a, A, L, T> CollectIntoHList for ValuesIterator<'a, LVCons<L, T>, A>
where
    L: Label,
    A: Adapter<&'a L::AssocType>,
    ValuesIterator<'a, T, A>: CollectIntoHList
{
    type Output = Cons<
        <A as Adapter<&'a L::AssocType>>::Output,
        <ValuesIterator<'a, T, A> as CollectIntoHList>::Output
    >;
    fn collect_into_hlist(self) -> Self::Output {
        let (item, next_iter) = self.next();
        Cons {
            head: item,
            tail: next_iter.collect_into_hlist()
        }
    }
}

impl<'a, A> CollectIntoHList for ConsIterator<'a, Nil, A> {
    type Output = Nil;
    fn collect_into_hlist(self) -> Self::Output { Nil }
}

impl<'a, A, H, T> CollectIntoHList for ConsIterator<'a, Cons<H, T>, A>
where
    A: Adapter<&'a H>,
    ConsIterator<'a, T, A>: CollectIntoHList
{
    type Output = Cons<
        <A as Adapter<&'a H>>::Output,
        <ConsIterator<'a, T, A> as CollectIntoHList>::Output
    >;
    fn collect_into_hlist(self) -> Self::Output {
        let (item, next_iter) = self.next();
        Cons {
            head: item,
            tail: next_iter.collect_into_hlist()
        }
    }
}

/// Collects an iterator (either [ConsIterator](struct.ConsIterator.html) or
/// [ValuesIterator](struct.ValuesIterator.html)) into a labeled hetereogeneous cons-list, assuming
/// the original iterated list had labels.
///
/// The resulting cons-list will maintain label information -- it typically returns
/// an [LVCons](../struct.Cons.html)-list. For a collecting into non-labeled lists, see
/// [CollectIntoHList](trait.CollectIntoHList.html).
pub trait CollectIntoLabeledHList {
    /// Output type of collected list
    type Output;
    /// Collects the contents of an iterator into a labeled cons-list
    fn collect_into_labeled_hlist(self) -> Self::Output;
}

impl<'a, A> CollectIntoLabeledHList for ConsIterator<'a, Nil, A> {
    type Output = Nil;
    fn collect_into_labeled_hlist(self) -> Self::Output { Nil }
}

impl<'a, A, L, T> CollectIntoLabeledHList for ConsIterator<'a, LVCons<L, T>, A>
where
    L: Label,
    A: Adapter<&'a Labeled<L>, Output=L::AssocType>,
    ConsIterator<'a, T, A>: CollectIntoHList
{
    type Output = LVCons<
        L,
        <ConsIterator<'a, T, A> as CollectIntoHList>::Output
    >;
    fn collect_into_labeled_hlist(self) -> Self::Output {
        let (item, next_iter) = self.next();
        Cons {
            head: Labeled::new(item),
            tail: next_iter.collect_into_hlist()
        }
    }
}

impl<'a, A> CollectIntoLabeledHList for ValuesIterator<'a, Nil, A> {
    type Output = Nil;
    fn collect_into_labeled_hlist(self) -> Self::Output { Nil }
}

impl<'a, A, L, T> CollectIntoLabeledHList for ValuesIterator<'a, LVCons<L, T>, A>
where
    L: Label,
    A: Adapter<&'a L::AssocType, Output=L::AssocType>,
    ValuesIterator<'a, T, A>: CollectIntoHList
{
    type Output = LVCons<
        L,
        <ValuesIterator<'a, T, A> as CollectIntoHList>::Output
    >;
    fn collect_into_labeled_hlist(self) -> Self::Output {
        let (item, next_iter) = self.next();
        Cons {
            head: Labeled::new(item),
            tail: next_iter.collect_into_hlist()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::iter::*;

    #[label(type=Vec<usize>)]
    struct Label1;

    #[label(type=Vec<&'static str>)]
    struct Label2;

    #[label(type=Vec<f64>)]
    struct Label3;

    #[test]
    fn iterate() {
        let test_list = lhlist![
            Label1 = vec![8usize, 4, 1, 5, 2],
            Label2 = vec!["Hello", "World!"],
            Label3 = vec![0.4f64, -3.5, 3.5, 0.3],
        ];

        let iter = test_list.iter_values();
        println!("{:?}", iter.collect_into_hlist());
    }

    #[test]
    fn map() {
        let test_list = lhlist![
            Label1 = vec![8usize, 4, 1, 5, 2],
            Label2 = vec!["Hello", "World!"],
            Label3 = vec![0.4f64, -3.5, 3.5, 0.3],
        ];

        #[derive(Debug)]
        struct DoStuff;
        impl MapFunc<&Vec<usize>> for DoStuff {
            type Output = usize;
            fn call(&mut self, item: &Vec<usize>) -> usize {
                item.iter().fold(0, |acc, value| acc + value)
            }
        }
        impl MapFunc<&Vec<&str>> for DoStuff {
            type Output = usize;
            fn call(&mut self, item: &Vec<&str>) -> usize {
                item.iter().fold(0, |acc, value| acc + value.len())
            }
        }
        impl MapFunc<&Vec<f64>> for DoStuff {
            type Output = usize;
            fn call(&mut self, item: &Vec<f64>) -> usize {
                item.len()
            }
        }

        let result = test_list.iter_values().map(DoStuff).collect_into_hlist();
        println!("{:?}", result);

        #[derive(Debug)]
        struct DoStuff2;

        impl MapFunc<usize> for DoStuff2 {
            type Output = usize;
            fn call(&mut self, item: usize) -> usize {
                item + 5
            }
        }

        let result = test_list.iter_values().map(DoStuff).map(DoStuff2).collect_into_hlist();
        println!("{:?}", result);
    }
}
