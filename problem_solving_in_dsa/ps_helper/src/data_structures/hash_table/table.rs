use std::fmt::Debug;

#[derive(Clone, PartialEq, Debug)]
pub enum Element<V>
where
    V: Clone + Debug + PartialEq,
{
    Value(V),
    Deleted,
    Empty,
}

impl<V> Element<V>
where
    V: Clone + Debug + PartialEq,
{
    pub fn is_value(&self) -> bool {
        match self {
            Element::Value(_) => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Element::Empty => true,
            _ => false,
        }
    }

    pub fn is_deleted(&self) -> bool {
        match self {
            Element::Deleted => true,
            _ => false,
        }
    }

    pub fn unwrap_value(&self) -> V {
        match self {
            Element::Value(v) => v.clone(),
            _ => panic!("Not a value"),
        }
    }
}

impl<V: Clone + Debug + PartialEq> Default for Element<V> {
    fn default() -> Self {
        Element::Empty
    }
}

#[derive(Default, Debug, Clone)]
/// Hash table that uses linear probing to resolve collisions
pub struct MyHashTable<K, V>
where
    K: Default + Clone + Debug + PartialEq,
    V: Default + Clone + Debug + PartialEq,
{
    table: Vec<Element<V>>,
    size: usize,
    phantom_data: std::marker::PhantomData<K>,
}
