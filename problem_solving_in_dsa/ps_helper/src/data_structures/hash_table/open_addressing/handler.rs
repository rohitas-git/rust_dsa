use std::{fmt::Debug, option};


#[derive(Debug, Default, Clone)]
pub struct LinearProbingHandler;

#[derive(Debug, Default, Clone)]
pub struct QuadraticProbingHandler;

#[derive(Debug, Clone)]
pub struct DoubleHashingHandler{
    // first_hasher: fn(u32, usize) -> usize,
    second_hasher: fn(u32, usize) -> usize,
}

pub trait OpenAddressing: CloneAsOpenAddressing + 'static {
    fn next_index(&self, key_hash: usize, table_size: usize, step: usize) -> usize;
}

impl OpenAddressing for LinearProbingHandler {
    fn next_index(&self, key_hash: usize, table_size: usize, step: usize) -> usize {
        (key_hash + step) % table_size
    }

}
impl OpenAddressing for QuadraticProbingHandler {
    fn next_index(&self, key_hash: usize, table_size: usize, step: usize ) -> usize {
        (key_hash + step * step) % table_size
    }
}
impl OpenAddressing for DoubleHashingHandler {
    fn next_index(&self, key_hash: usize, table_size: usize, step: usize) -> usize {
        let second_key_hash = todo!("double hashing");
        // (key_hash + step * second_key_hash) % table_size;
        todo!("double hashing")
    }
}

// new “helper” super trait to help with cloning
pub trait CloneAsOpenAddressing {
    fn clone_as_open_addressing(&self) -> Box<dyn OpenAddressing>;
}
// with a blanket impl:
impl<T: OpenAddressing + Clone> CloneAsOpenAddressing for T {
    fn clone_as_open_addressing(&self) -> Box<dyn OpenAddressing> {
        Box::new(self.clone())
    }
}

impl Debug for dyn OpenAddressing {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "using {}", stringify!(self))
    }
}

