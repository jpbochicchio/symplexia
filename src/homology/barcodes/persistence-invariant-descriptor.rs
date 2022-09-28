#![allow(dead_code)]
pub mod persistence_invariant_descriptor {
    use std::collections::hash_map::Entry::{Occupied, Vacant};
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::hash::Hash;

    #[derive(Debug, Clone, PartialEq)]
    pub struct PersistenceInvariantDescriptor<I, G> {
        pub intervals: HashMap<u32, Vec<I>>,
        pub generators: HashMap<u32, Vec<G>>,
        pub interval_generator_pairs: HashMap<u32, Vec<(I, G)>>,
    }

    impl<I, G> PersistenceInvariantDescriptor<I, G>
    where
        I: Clone + Debug,
        G: Clone + Debug,
    {
        pub fn new() -> Self {
            Self {
                intervals: HashMap::new(),
                generators: HashMap::new(),
                interval_generator_pairs: HashMap::new()
            }
        }

        pub fn add_interval(&mut self, dimension: u32, interval: I, generator: G) {
            Self::insert_to_vec_if_exists(dimension, interval.clone(), &mut self.intervals);
            Self::insert_to_vec_if_exists(dimension, generator.clone(), &mut self.generators);
            Self::insert_to_vec_if_exists(dimension, (interval, generator), &mut self.interval_generator_pairs);
        }

        pub fn get_dimensions(self) -> Vec<u32> {
            let mut dim_set: Vec<u32> = Vec::new();
            
            self.intervals.keys().for_each(|k| {
                dim_set.push(*k);
            });

            return dim_set;
        }

        pub fn get_intervals_at_dimension(mut self, dimension: u32) -> Vec<I> {
            Self::get_or_default(dimension, &mut self.intervals)
        }

        pub fn get_generators_at_dimension(mut self, dimension: u32) -> Vec<G> {
            Self::get_or_default(dimension, &mut self.generators)
        }

        pub fn get_interval_generator_pairs_at_dimension(mut self, dimension: u32) -> Vec<(I, G)> {
            Self::get_or_default(dimension, &mut self.interval_generator_pairs)
        }

        fn insert_to_vec_if_exists<T, U>(k: T, v: U, map: &mut HashMap<T, Vec<U>>) where U: Clone, T: Eq + Hash {
            match map.entry(k) {
                Vacant(e) => { e.insert(vec![v.clone()]); },
                Occupied(mut e) => { e.get_mut().push(v); }
            }   
        }

        fn get_or_default<T, U>(k: T, map: &mut HashMap<T, Vec<U>>) -> Vec<U> where U: Clone, T: Eq + Hash {
            match map.entry(k) {
                Vacant(_) => { Vec::new() },
                Occupied(e) => { e.get().clone() }
            }
        }
    }
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_invariant_initialization() {
        // TODO -- will implement tests when implementing generators
    }
}
