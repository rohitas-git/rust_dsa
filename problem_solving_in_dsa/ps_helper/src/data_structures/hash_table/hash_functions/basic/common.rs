use crate::data_structures::hash_table::traits::HashStrategy;
use std::hash::{Hash, Hasher};


#[derive(Clone)]
pub struct ModularHash;
impl HashStrategy<u32> for ModularHash {
    fn hash(&self, key: &u32, table_capacity: usize) -> usize {
        modular_hash(key, table_capacity)
    }
}

#[derive(Clone)]
pub struct MultiplicativeHash;
impl HashStrategy<u32> for MultiplicativeHash {
    fn hash(&self, key: &u32, table_capacity: usize) -> usize {
        multiplicative_hash(key, table_capacity)
    }
}

// Example implementation for String
#[derive(Clone)]
pub struct StringHash;
impl HashStrategy<String> for StringHash {
    fn hash(&self, key: &String, table_capacity: usize) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % table_capacity
    }
}

pub struct SipHasherStrategy;
impl HashStrategy<u32> for SipHasherStrategy {
    fn hash(&self, key: &u32, table_capacity: usize) -> usize {
        let mut hasher = std::hash::SipHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % table_capacity
    }
}

// Implement a generic hasher strategy
#[derive(Clone)]
pub struct GenericHasherStrategy<H: Hasher + Default + Clone> {
    _phantom: std::marker::PhantomData<H>,
}
impl<H: Hasher + Default + Clone> GenericHasherStrategy<H> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<K: Hash, H: Hasher + Default + Clone> HashStrategy<K> for GenericHasherStrategy<H> {
    fn hash(&self, key: &K, table_capacity: usize) -> usize {
        let mut hasher = H::default();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % table_capacity
    }
}

/// computed as target % table_capacity
pub fn modular_hash(target: &u32, table_capacity: usize) -> usize {
    *target as usize % table_capacity
}

/// computed as floor[m * frac(ka)]
/// m = table_capacity
/// ka = target * a
/// a = 0.314159
pub fn multiplicative_hash(target: &u32, table_capacity: usize) -> usize {
    let a = 0.314159_f32;
    let ka = *target as f32 * a;
    let frac_part = ka % 1 as f32;
    return (table_capacity as f32 * frac_part).floor() as usize;
}

/// computed: (str[0]*(x^0) + str[1]*(x^1) + str[2]*(x^2) + str[3]*(x^4)) % m ,
/// where x = 33
pub fn weighted_string_hash(target: &str, table_capacity: usize) -> usize {
    let mut sum = 0;
    let x = 33_usize;
    for (i, c) in target.chars().enumerate() {
        sum += c as usize * x.pow(i as u32);
    }
    return sum % table_capacity;
}
