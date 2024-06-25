use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

pub struct Counter<T>
where
    T: Hash + Eq,
{
    map: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Hash + Eq,
{
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut counter = Counter {
            map: HashMap::new(),
        };

        for item in iter {
            *counter.map.entry(item).or_insert(0) += 1;
        }

        counter
    }

    pub fn add(&mut self, item: T) {
        *self.map.entry(item).or_insert(0) += 1;
    }

    pub fn get(&self, item: &T) -> Option<&usize> {
        self.map.get(item)
    }

    pub fn most_common(&self) -> Option<(&T, &usize)> {
        self.map.iter().max_by_key(|&(_, count)| *count)
    }

    // Add iter method to Counter
    pub fn iter(&self) -> CounterRefIterator<T> {
        CounterRefIterator {
            inner: self.map.iter(),
        }
    }

    // Add iter_mut method to Counter
    pub fn iter_mut(&mut self) -> CounterMutIterator<T> {
        CounterMutIterator {
            inner: self.map.iter_mut(),
        }
    }
}

// Define an iterator for borrowed Counter
pub struct CounterRefIterator<'a, T>
where
    T: Hash + Eq,
{
    inner: std::collections::hash_map::Iter<'a, T, usize>,
}

impl<'a, T> Iterator for CounterRefIterator<'a, T>
where
    T: Hash + Eq,
{
    type Item = (&'a T, &'a usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

// Implement IntoIterator for &Counter
impl<'a, T> IntoIterator for &'a Counter<T>
where
    T: Hash + Eq,
{
    type Item = (&'a T, &'a usize);
    type IntoIter = CounterRefIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        CounterRefIterator {
            inner: self.map.iter(),
        }
    }
}

// Define an iterator for owned Counter
pub struct CounterIntoIterator<T>
where
    T: Hash + Eq,
{
    inner: std::collections::hash_map::IntoIter<T, usize>,
}

impl<T> Iterator for CounterIntoIterator<T>
where
    T: Hash + Eq,
{
    type Item = (T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

// Implement IntoIterator for Counter
impl<T> IntoIterator for Counter<T>
where
    T: Hash + Eq,
{
    type Item = (T, usize);
    type IntoIter = CounterIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        CounterIntoIterator {
            inner: self.map.into_iter(),
        }
    }
}

// Define a mutable iterator for Counter
pub struct CounterMutIterator<'a, T>
where
    T: Hash + Eq,
{
    inner: std::collections::hash_map::IterMut<'a, T, usize>,
}

impl<'a, T> Iterator for CounterMutIterator<'a, T>
where
    T: Hash + Eq,
{
    type Item = (&'a T, &'a mut usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

// Implement IntoIterator for &mut Counter
impl<'a, T> IntoIterator for &'a mut Counter<T>
where
    T: Hash + Eq,
{
    type Item = (&'a T, &'a mut usize);
    type IntoIter = CounterMutIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        CounterMutIterator {
            inner: self.map.iter_mut(),
        }
    }
}
