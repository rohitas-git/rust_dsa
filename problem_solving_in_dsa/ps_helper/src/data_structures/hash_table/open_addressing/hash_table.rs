use super::super::hash_functions::basic::common::modular_hash;
use super::super::table::Element;
use super::super::traits::HashTable;
use super::handler::*;
use std::fmt::Debug;

use Element::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct HashNode<K, V>
where
    K: Default + Clone + Debug + PartialEq,
    V: Default + Clone + Debug + PartialEq,
{
    key: Option<K>,
    value: Element<V>,
}

impl<K, V> HashNode<K, V>
where
    K: Default + Clone + Debug + PartialEq,
    V: Default + Clone + Debug + PartialEq,
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

#[derive(Debug)]
/// Hash table that uses linear probing to resolve collisions
pub struct MyHashTable<K, V>
where
    K: Default + Clone + Debug + PartialEq,
    V: Default + Clone + Debug + PartialEq,
{
    table: Vec<HashNode<K, V>>,
    size: usize,
    collision_handler: Box<dyn OpenAddressing>,
    key_hasher: fn(&K, usize) -> usize,
}

impl<K, V> Clone for MyHashTable<K, V>
where
    K: Default + Clone + Debug + PartialEq,
    V: Default + Clone + Debug + PartialEq,
{
    fn clone(&self) -> Self {
        Self {
            table: self.table.clone(),
            size: self.size,
            collision_handler: self.collision_handler.clone_as_open_addressing(),
            key_hasher: self.key_hasher,
        }
    }
}

impl<K, V> MyHashTable<K, V>
where
    K: Default + Clone + Debug + PartialEq,
    V: Default + Clone + Debug + PartialEq,
{
    fn next_index(&self, key_hash: usize, table_size: usize, step: usize) -> usize {
        self.collision_handler
            .next_index(key_hash, table_size, step)
    }

    pub fn set_linear_probing(&mut self) {
        self.collision_handler = Box::new(LinearProbingHandler::default());
    }

    pub fn set_double_hashing(&mut self) {
        // self.collision_handler = Box::new(DoubleHashingHandler::default());
        todo!("not implemented yet");
    }

    pub fn set_quadratic_probing(&mut self) {
        self.collision_handler = Box::new(QuadraticProbingHandler::default());
    }

    pub fn set_key_hasher(&mut self, key_hasher: fn(&K, usize) -> usize) {
        self.key_hasher = key_hasher;
    }
}

impl<V> HashTable for MyHashTable<u32, V>
where
    V: Default + Clone + Debug + PartialEq,
{
    type KeyType = u32;
    type ValueType = V;

    fn new(capacity: usize) -> Self {
        Self {
            table: vec![HashNode::default(); capacity],
            size: 0,
            collision_handler: Box::new(LinearProbingHandler::default()),
            key_hasher: modular_hash,
        }
    }

    fn capacity(&self) -> usize {
        self.table.capacity()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn hash(&self, key: &Self::KeyType) -> usize {
        (self.key_hasher)(key, self.capacity())
    }

    /// Returns the value mapped to the key in hash table
    fn get_value(&self, key: &Self::KeyType) -> Option<&Element<Self::ValueType>> {
        if !self.contains_key(&key) {
            return None;
        }
        let key_hash = self.hash(key);

        // Search
        let mut index = key_hash;
        let mut step = 0;
        while !self.table[index].value.is_empty() {
            if self.table[index].value.is_value() && self.table[index].key == Some(*key) {
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
        let table = &self.table;
        let key_hash = self.hash(target);
        let mut index = key_hash;
        let mut step = 0;

        if self.size() == 0 {
            return false;
        }

        // End the search when
        // - we find the key
        // - find empty slot
        // - transversed whole table

        // While not found empty slot
        while table[index].value != Empty {
            if table[index].value.is_value() && table[index].key == Some(*target) {
                return true;
            }
            step += 1;
            index = self.next_index(key_hash, cap, step);

            // when transversed whole table
            if index == key_hash {
                return false;
            }
        }
        true
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    fn insert(&mut self, key: &Self::KeyType, value: &Self::ValueType) -> Option<Self::ValueType> {
        if self.capacity() == self.size() {
            self.table.resize(2 * self.capacity(), HashNode::default());
        }

        // If key exists, update value and return old
        // If key does not exist, insert and return None
        if !self.contains_key(key) {
            let key_hash = self.hash(key);
            let mut index = key_hash;
            let mut step = 0;
            while self.table[index].value != Empty {
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
            while self.table[index].value != Empty {
                if self.table[index].value.is_value() && self.table[index].key == Some(*key) {
                    let old = self.table[index].value.clone();
                    self.table[index].value = Value(value.clone());
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

        while self.table[index].value != Empty {
            if self.table[index].value.is_value() && self.table[index].key == Some(*key) {
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
