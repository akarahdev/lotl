use std::hash::Hash;

/// Represents an AST element with an associated ID.
pub trait Tagged {
    /// The associated tag that this type is tagged with
    type TagType: Sized + Tag;
    /// Returns the ID associated with this element.
    fn id(&self) -> &Self::TagType;
}

/// Generates a new AST tag from an input.
pub trait Tag: Hash + PartialEq + Eq + Clone {
    /// The input to generate a tag from.
    type Input: Sized;
    /// Creates a new instance of the tag.
    fn make_new_from(input: &Self::Input) -> Self;
}

/// Creates a special `make_new` function for `Tag` with an input of `()`
pub trait PureTag: Tag<Input = ()> {
    /// Creates a new instance of the tag. This should be preferred over `make_new_from` where possible.
    fn make_new() -> Self
    where
        Self: Sized,
    {
        Self::make_new_from(&())
    }
}
impl<T: Tag<Input = ()>> PureTag for T {}
