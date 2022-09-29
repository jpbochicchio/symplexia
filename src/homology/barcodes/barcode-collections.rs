#![allow(dead_code)]
pub mod barcode_collections {
    use std::fmt::Debug;

    use crate::intervals::intervals::Interval;
    use crate::persistence_invariant_descriptor::persistence_invariant_descriptor::PersistenceInvariantDescriptor;

    #[derive(Clone)]
    pub struct AnnotatedBarcodeCollection<T, G> {
        use_left_closed_default: bool,
        use_right_closed_default: bool,
        persistence_invariant_descriptor: PersistenceInvariantDescriptor<Interval<T>, G>,
    }

    impl<T, G> AnnotatedBarcodeCollection<T, G>
    where
        T: Clone + Debug + PartialEq + PartialOrd,
        G: Clone + Debug,
    {
        pub fn new(
            use_left_closed_default: bool,
            use_right_closed_default: bool,
            persistence_invariant_descriptor: PersistenceInvariantDescriptor<Interval<T>, G>,
        ) -> Self {
            Self {
                use_left_closed_default,
                use_right_closed_default,
                persistence_invariant_descriptor,
            }
        }

        pub fn get_infinite_intervals(&mut self) -> Self {
            let mut result: AnnotatedBarcodeCollection<T, G> =
                AnnotatedBarcodeCollection::default();

            self.persistence_invariant_descriptor
                .intervals
                .keys()
                .for_each(|dim| {
                    let interval_list: &Vec<Interval<T>> = self
                        .persistence_invariant_descriptor
                        .intervals
                        .get(dim)
                        .unwrap();
                    let generator_list: &Vec<G> = self
                        .persistence_invariant_descriptor
                        .generators
                        .get(dim)
                        .unwrap();

                    for i in 0..interval_list.len() {
                        let interval: &Interval<T> = interval_list.get(i).unwrap();
                        let generator: &G = generator_list.get(i).unwrap();

                        if interval.clone().is_infinite() {
                            result.persistence_invariant_descriptor.add_interval(
                                *dim,
                                interval.clone(),
                                generator.clone(),
                            );
                        }
                    }
                });

            return result;
        }

        pub fn filter_by_max_dimension(mut self, max_dim: u32) -> Self {
            let mut result: AnnotatedBarcodeCollection<T, G> =
                AnnotatedBarcodeCollection::get_infinite_intervals(&mut self);

            // It's absurd how much cleaner this is than the original version.
            result
                .persistence_invariant_descriptor
                .intervals
                .retain(|k, _| *k < max_dim);
            result
                .persistence_invariant_descriptor
                .generators
                .retain(|k, _| *k < max_dim);
            result
                .persistence_invariant_descriptor
                .interval_generator_pairs
                .retain(|k, _| *k < max_dim);

            return result;
        }

        pub fn add_interval(&mut self, dimension: u32, start: T, end: T, generating_cycle: G) {
            self.persistence_invariant_descriptor.add_interval(
                dimension,
                Interval::new(
                    Some(start),
                    Some(end),
                    self.use_left_closed_default,
                    self.use_right_closed_default,
                    false,
                    false,
                ),
                generating_cycle,
            );
        }

        pub fn add_right_infinite_interval(
            &mut self,
            dimension: u32,
            start: T,
            generating_cycle: G,
        ) {
            self.persistence_invariant_descriptor.add_interval(
                dimension,
                Interval::new(
                    Some(start),
                    None::<T>,
                    self.use_left_closed_default,
                    self.use_right_closed_default,
                    false,
                    true,
                ),
                generating_cycle,
            );
        }

        pub fn add_left_infinite_interval(&mut self, dimension: u32, end: T, generating_cycle: G) {
            self.persistence_invariant_descriptor.add_interval(
                dimension,
                Interval::new(
                    None::<T>,
                    Some(end),
                    self.use_left_closed_default,
                    self.use_right_closed_default,
                    true,
                    false,
                ),
                generating_cycle,
            );
        }
    }

    impl<T, G> Default for AnnotatedBarcodeCollection<T, G>
    where
        T: Clone + Debug + PartialEq + PartialOrd,
        G: Clone + Debug,
    {
        fn default() -> Self {
            Self {
                use_left_closed_default: true,
                use_right_closed_default: false,
                persistence_invariant_descriptor: PersistenceInvariantDescriptor::new(),
            }
        }
    }
}
