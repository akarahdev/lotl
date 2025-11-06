use crate::ids::{PureTag, Tag, Tagged};
use std::collections::HashMap;

/// A graph matching node IDs to their values.
#[derive(Debug, Clone)]
pub struct IdGraph<T: Tagged> {
    map: HashMap<T::TagType, T>,
}

impl<T: Tagged> Default for IdGraph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Tagged> IdGraph<T> {
    /// Creates a new empty ID graph.
    pub fn new() -> Self {
        IdGraph {
            map: HashMap::new(),
        }
    }

    /// Registers a tag and the node into the graph, with the given input as a seed.
    pub fn register_with<F>(&mut self, input: &<T::TagType as Tag>::Input, f: F) -> T::TagType
    where
        F: FnOnce(T::TagType) -> T,
    {
        let tag = T::TagType::make_new_from(input);
        self.map.insert(tag.clone(), f(tag.clone()));
        tag
    }

    /// Extends this ID graph based on an iterator.
    pub fn extend<I: Iterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.map.insert(item.id().clone(), item);
        }
    }

    /// Exposes all keys in this ID graph as an iterator
    pub fn keys(&self) -> impl Iterator<Item = &T::TagType> {
        self.map.keys()
    }

    /// Exposes all values in this ID graph as an iterator
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.map.values()
    }

    /// Exposes all values in this ID graph as an iterator, consuming the graph
    pub fn into_values(self) -> impl Iterator<Item = T> {
        self.map.into_values()
    }

    /// Exposes all pairs in this ID graph as an iterator
    pub fn iter(&self) -> impl Iterator<Item = (&T::TagType, &T)> {
        self.map.iter()
    }
}

impl<T: Tagged> IdGraph<T>
where
    T::TagType: PureTag,
{
    /// Registers a tag and node into the graph.
    pub fn register<F: FnOnce(T::TagType) -> T>(&mut self, f: F) -> T::TagType {
        let tag = T::TagType::make_new();
        self.map.insert(tag.clone(), f(tag.clone()));
        tag
    }
}
