pub mod simplex {
    use std::ops::Range;
    use std::sync::Mutex;

    use crate::primitive_basis::primitives::PrimitiveBasisElement;
    use crate::homology_utility::homology_utility::HomologyUtility;

    lazy_static! {
        static ref SHARED_HOMOLOGY_UTILITY: Mutex<HomologyUtility> = Mutex::new(HomologyUtility::new());
    }

    #[derive(Clone)]
    pub struct Simplex {
        vertices: Vec<i32>
    }

    impl Simplex {
        pub fn new(mut vertices: Vec<i32>) -> Self {
            vertices.sort();
            Self {
                vertices
            }
        }

        pub fn from_ran(ran: Range<i32>) -> Self {
            let mut ran_as_vec: Vec<i32> = ran.collect();
            return Simplex::new(ran_as_vec);
        }
    }

    impl PrimitiveBasisElement for Simplex {
        fn get_dimension(self) -> i32 {
            return self.vertices.len() as i32 - 1
        }

        // Arrays implement Copy and Clone, while also being closer to the 
        // original implementation. Maybe I should rewrite what I currently
        // have and use arrays instead of vectors;
        fn get_boundary_array(self) -> Vec<Self> where Self: Sized {
            if self.clone().get_dimension() == 0 {
                return vec![Simplex::new(Vec::new())];
            }

            let mut boundary_array: Vec<Simplex> = vec![Simplex::new(vec![0])];
            for i in 0..self.vertices.clone().len() {
                boundary_array[i as usize] = Simplex::new(HomologyUtility::remove_index(self.vertices.clone(), i as u32));
            }

            return boundary_array
        }

        fn get_boundary_coefficients(self) -> Vec<i32> {
            if self.vertices.len() == 1 {
                return SHARED_HOMOLOGY_UTILITY.lock().unwrap().get_default_boundary_coefficients(0);
            }

            return SHARED_HOMOLOGY_UTILITY.lock().unwrap().get_default_boundary_coefficients(self.vertices.len() as u32);
        }
    }
}