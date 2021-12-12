use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct CustomSet<T: PartialEq + Clone + std::hash::Hash + std::cmp::Eq> {
    // This field is here to make the template compile and not to
    // complain about unused type parameter 'T'. Once you start
    // solving the exercise, delete this field and the 'std::marker::PhantomData'
    // import.
    set: HashSet::<T>
}

impl<T: PartialEq + Clone + std::hash::Hash + std::cmp::Eq> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut res = CustomSet {
            set: HashSet::new()
        };
        _input.iter().for_each(|elm| res.add(elm.clone()));
        res
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.set.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.set.insert(_element);
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.set.iter().all(|elm| _other.set.contains(elm))
    }

    pub fn is_empty(&self) -> bool {
        self.set.len() == 0
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        !_other.set.iter().any(|elm| self.set.contains(elm))
    }

    pub fn intersection(&self, _other: &Self) -> Self {
        CustomSet {
            set: _other.set.iter().filter(|elm| self.set.contains(elm)).cloned().collect()
        }
    }

    pub fn difference(&self, _other: &Self) -> Self {
        CustomSet {
            set: _other.set.iter().filter(|elm| !self.set.contains(elm)).cloned().collect()
        }
    }

    pub fn union(&self, _other: &Self) -> Self {
        let mut res = Self::new(&[]);
        self.set.iter().for_each(|elm| res.add(elm.clone()));
        _other.set.iter().for_each(|elm| res.add(elm.clone()));
        res
    }
}
