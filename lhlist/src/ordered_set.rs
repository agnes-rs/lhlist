/// The end of a heterogeneous ordered set.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Empty;

/// Main buildling block of a heterogeneous set.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct OrderedSet<H, T> {
    /// Value of this element of the set.
    pub head: H,
    /// Remaining elements of the set.
    pub tail: T,
}
