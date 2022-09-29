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

    // JavaPlex implements a few more methods, but they are never called
    // Hence, we will omit them here
    impl<T, G> AnnotatedBarcodeCollection<T, G>
    where
        T: Clone + Debug + PartialEq + PartialOrd,
        G: Clone + Debug + Default,
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

        pub fn add_interval(
            &mut self,
            dimension: u32,
            start: T,
            end: T,
            generating_cycle: Option<G>,
        ) {
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
                generating_cycle.unwrap_or_default(),
            );
        }

        pub fn add_right_infinite_interval(
            &mut self,
            dimension: u32,
            start: T,
            generating_cycle: Option<G>,
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
                generating_cycle.unwrap_or_default(),
            );
        }

        pub fn add_left_infinite_interval(
            &mut self,
            dimension: u32,
            end: T,
            generating_cycle: Option<G>,
        ) {
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
                generating_cycle.unwrap_or_default(),
            );
        }
    }

    impl<T, G> Default for AnnotatedBarcodeCollection<T, G>
    where
        T: Clone + Debug + PartialEq + PartialOrd,
        G: Clone + Debug + Default,
    {
        fn default() -> Self {
            Self {
                use_left_closed_default: true,
                use_right_closed_default: false,
                persistence_invariant_descriptor: PersistenceInvariantDescriptor::new(),
            }
        }
    }

    pub struct BarcodeCollection<T, G> {
        annotated_collection: AnnotatedBarcodeCollection<T, G>,
    }

    // The JavaPlex version of this struct is only parameterized by a single generic, but I am not sure
    // how to achieve this in Rust. Would need some high level type that could stand in for G. As a side
    // effect, I'm not sure how to implement the "forgetGeneratorType" method from the JavaPlex class.
    impl<T, G> BarcodeCollection<T, G>
    where
        T: Clone + Debug + PartialEq + PartialOrd,
        G: Clone + Debug + Default,
    {
        pub fn new(annotated_collection: AnnotatedBarcodeCollection<T, G>) -> Self {
            Self {
                annotated_collection,
            }
        }

        pub fn add_interval(&mut self, dimension: u32, start: T, end: T) {
            self.annotated_collection
                .add_interval(dimension, start, end, None::<G>);
        }

        pub fn add_right_infinite_interval(&mut self, dimension: u32, start: T) {
            self.annotated_collection
                .add_right_infinite_interval(dimension, start, None::<G>);
        }

        pub fn add_left_infinite_interval(&mut self, dimension: u32, end: T) {
            self.annotated_collection
                .add_left_infinite_interval(dimension, end, None::<G>);
        }

        pub fn add_direct_interval(&mut self, dimension: u32, direct_interval: Interval<T>) {
            self.add_interval(
                dimension,
                direct_interval.clone().get_start().unwrap(),
                direct_interval.get_end().unwrap().clone(),
            );
        }
    }

    impl<T, G> Default for BarcodeCollection<T, G>
    where
        T: Clone + Debug + PartialEq + PartialOrd,
        G: Clone + Debug + Default,
    {
        fn default() -> Self {
            Self {
                annotated_collection: AnnotatedBarcodeCollection::default(),
            }
        }
    }
}
