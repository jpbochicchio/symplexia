pub mod simplex {
    use std::ops::Range;

    use crate::primitive_basis::primitives::PrimitiveBasisElement;

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

        fn get_boundary_array(self) -> Vec<Self> where Self: Sized {
            todo!()
        }

        fn get_boundary_coefficients(self) -> Vec<i32> {
            todo!()
        }
    }
}