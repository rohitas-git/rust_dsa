use std::fmt::Debug;
use super::table::Element;
pub trait HashTable {
    type KeyType: Clone + Debug + Default;
    type ValueType: Clone + Debug + Default + PartialEq;
    
    fn size(&self) -> usize;
    fn capacity(&self) -> usize;
    fn hash(&self, target: &Self::KeyType) -> usize;
    fn get_value(&self, key: &Self::KeyType) -> Option<&Element<Self::ValueType>>;
    fn contains_key(&self, target: &Self::KeyType) -> bool;
    fn insert(&mut self, key: &Self::KeyType, value: &Self::ValueType) -> Option<Self::ValueType>;
    fn delete(&mut self, key: &Self::KeyType) -> bool;
}
