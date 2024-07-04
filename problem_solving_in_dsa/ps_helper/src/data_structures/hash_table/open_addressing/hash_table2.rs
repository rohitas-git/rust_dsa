use super::super::hash_functions::basic::common::*;
use super::super::table::Element;
use super::super::traits::HashStrategy;
use super::super::traits::HashTable;
use super::handler::*;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use Element::*;

trait KeyTraitBounds: Default + Clone + Debug + PartialEq + Into<u8> {}
trait ValueTraitBounds: Default + Clone + Debug + PartialEq {}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct HashNode<K, V>
where
    K: KeyTraitBounds,
    V: ValueTraitBounds,
{
    key: Option<K>,
    value: Element<V>,
}

impl<K, V> HashNode<K, V>
where
    K: KeyTraitBounds,
    V: ValueTraitBounds,
{
    fn delete(&mut self) {
        self.key = None;
        self.value = Empty;
    }

    fn new(key: K, value: V) -> Self {
        Self {
            key: Some(key),
            value: Value(value),
        }
    }
}

pub struct OpenAddresserHashTable<K, V, S, H>
where
    K: KeyTraitBounds + Into<u8>,
    V: ValueTraitBounds,
    S: OpenAddressing + Clone,
    H: HashStrategy<K>,
{
    table: Vec<HashNode<K, V>>,
    size: usize,
    collision_handler: S,
    key_hasher: H,
}

impl<K, V, S, H> Clone for OpenAddresserHashTable<K, V, S, H>
where
    K: KeyTraitBounds,
    V: ValueTraitBounds,
    S: OpenAddressing + Clone,
    H: HashStrategy<K> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            size: self.size,
            collision_handler: self.collision_handler.clone(),
            key_hasher: self.key_hasher.clone(),
        }
    }
}

impl<K, V, S, H> OpenAddresserHashTable<K, V, S, H>
where
    K: KeyTraitBounds,
    V: ValueTraitBounds,
    S: OpenAddressing + Clone,
    H: HashStrategy<K>,
{
    pub fn new(capacity: usize, collision_handler: S, key_hasher: H) -> Self {
        Self {
            table: vec![HashNode::default(); capacity],
            size: 0,
            collision_handler,
            key_hasher,
        }
    }

    fn next_index(&self, key_hash: usize, table_size: usize, step: usize) -> usize {
        self.collision_handler
            .next_index(key_hash, table_size, step)
    }

    // pub fn set_linear_probing(&mut self) {
    //     self.collision_handler = Box::new(LinearProbingHandler::default());
    // }

    // pub fn set_double_hashing(&mut self) {
    //     // self.collision_handler = Box::new(DoubleHashingHandler::default());
    //     todo!("not implemented yet");
    // }

    // pub fn set_quadratic_probing(&mut self) {
    //     self.collision_handler = Box::new(QuadraticProbingHandler::default());
    // }

    pub fn set_key_hasher(&mut self, key_hasher: H) {
        self.key_hasher = key_hasher;
    }
}

impl<K, V, S, H> HashTable for OpenAddresserHashTable<K, V, S, H>
where
    K: KeyTraitBounds + Into<u8>,
    V: ValueTraitBounds,
    S: OpenAddressing + Clone,
    H: HashStrategy<K>,
{
    type KeyType = K;
    type ValueType = V;

    fn capacity(&self) -> usize {
        self.table.capacity()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn hash(&self, key: &Self::KeyType) -> usize {
        self.key_hasher.hash(key, self.capacity())
    }

    /// Returns the value mapped to the key in hash table
    fn get_value(&self, key: &Self::KeyType) -> Option<&Element<Self::ValueType>> {
        if !self.contains_key(key) {
            return None;
        }
        let key_hash = self.hash(key);

        let mut index = key_hash;
        let mut step = 0;
        while !self.table[index].value.is_empty() {
            if self.table[index].value.is_value() && self.table[index].key == Some(key.clone()) {
                return Some(&self.table[index].value);
            }
            step += 1;
            index = self.next_index(key_hash, self.capacity(), step);
        }
        None
    }

    /// Returns true if key is used in this table
    fn contains_key(&self, target: &Self::KeyType) -> bool {
        let cap = self.capacity();
        let key_hash = self.hash(target);
        let mut index = key_hash;
        let mut step = 0;

        while self.table[index].value != Element::Empty {
            if self.table[index].value.is_value() && self.table[index].key == Some(target.clone()) {
                return true;
            }
            step += 1;
            index = self.next_index(key_hash, cap, step);

            if index == key_hash {
                return false;
            }
        }
        false
    }

    fn insert(&mut self, key: &Self::KeyType, value: &Self::ValueType) -> Option<Self::ValueType> {
        if self.capacity() == self.size() {
            self.table.resize(2 * self.capacity(), HashNode::default());
        }

        if !self.contains_key(key) {
            let key_hash = self.hash(key);
            let mut index = key_hash;
            let mut step = 0;
            while self.table[index].value != Element::Empty {
                step += 1;
                index = self.next_index(key_hash, self.capacity(), step);
            }
            self.table[index] = HashNode::new(key.clone(), value.clone());
            self.size += 1;
            None
        } else {
            let key_hash = self.hash(key);
            let mut index = key_hash;
            let mut step = 0;
            while self.table[index].value != Element::Empty {
                if self.table[index].value.is_value() && self.table[index].key == Some(key.clone())
                {
                    let old = self.table[index].value.clone();
                    self.table[index].value = Element::Value(value.clone());
                    return Some(old.unwrap_value());
                }
                step += 1;
                index = self.next_index(key_hash, self.capacity(), step);
            }
            None
        }
    }

    fn delete(&mut self, key: &Self::KeyType) -> bool {
        let cap = self.capacity();
        let key_hash = self.hash(key);
        let mut step = 0;
        let mut index = key_hash;

        if !self.contains_key(key) {
            return false;
        }

        while self.table[index].value != Element::Empty {
            if self.table[index].value.is_value() && self.table[index].key == Some(key.clone()) {
                self.table[index].delete();
                self.size -= 1;
                return true;
            }
            step += 1;
            index = self.next_index(index, cap, step);
            if index == key_hash {
                break;
            }
        }
        false
    }
}

// #[cfg(test)]
// mod test_my_hashtable {
//     use super::*;

//     #[test]
//     fn test_new_impl() {
//         let t = MyHash::new(7);
//         let r = MyHash {
//             table: vec![Empty; 7],
//             size: 0,
//         };
//         assert_eq!(t, r);
//         assert_eq!(t.capacity(), r.capacity());
//         assert_eq!(t.size(), r.size());
//     }

//     #[test]
//     fn test_search_impl() {
//         let t = MyHash {
//             table: vec![Empty, Empty, Empty, Value(10), Empty, Empty, Empty],
//             size: 0,
//         };

//         let res = t.search(10);
//         assert_eq!(res, true);

//         let res = t.search(17);
//         assert_eq!(res, false);
//     }

//     #[test]
//     fn test_insert_impl() {
//         let mut t = MyHash::new(7);

//         assert_eq!(t.insert(10), true);
//         let r = MyHash {
//             table: vec![Empty, Empty, Empty, Value(10), Empty, Empty, Empty],
//             size: 0,
//         };
//         assert_eq!(t, r);

//         assert_eq!(t.insert(17), true);
//         let r = MyHash {
//             table: vec![Empty, Empty, Empty, Value(10), Value(17), Empty, Empty],
//             size: 0,
//         };
//         assert_eq!(t, r);

//         assert_eq!(t.insert(1), true);
//         assert_eq!(t.insert(21), true);
//         assert_eq!(t.insert(21), false); // Already inserted
//         assert_eq!(t.insert(14), true);
//         assert_eq!(t.insert(15), true);
//         assert_eq!(t.insert(27), true);
//         assert_eq!(t.insert(12), false); // Filled
//     }

//     #[test]
//     fn test_delete_impl() {
//         let mut t = MyHash::new(7);
//         t.insert(10);
//         t.insert(2);
//         t.insert(3);
//         t.insert(4);
//         t.insert(5);
//         t.insert(6);
//         t.insert(7);

//         assert_eq!(t.delete(10), true);
//         assert_eq!(t.delete(2), true);
//         assert_eq!(t.delete(2), false);
//         assert_eq!(t.delete(99), false);
//         println!("{:?}", t);
//     }
// }
