/*
Might want to reference this utility with MutStatic

#[macro_use]
extern crate lazy_static;
extern crate mut_static;
use mut_static::MutStatic;
*/

pub mod homology_utility {
    pub struct HomologyUtility {
        default_boundary_coefficients: Vec<Vec<i32>>,
        default_coefficient_cache_size: u32,
    }

    impl HomologyUtility {
        pub fn new() -> Self {
            Self {
                default_boundary_coefficients: Vec::new(),
                default_coefficient_cache_size: 10
            }
        }

       /**
        * This function initializes the set of default boundary coefficients. It
        * constructs an array of the form
        * [[],
        *  [1],
        *  [1, -1],
        *  [1, -1, 1]
        *  [1, -1, 1, -1],
        *  ... ]
        */
        fn initialize_default_boundary_coefficients(&mut self) {
            for i in 0..self.default_coefficient_cache_size {
                self.default_boundary_coefficients[i as usize] = Vec::new();
                for j in 0..i {
                    self.default_boundary_coefficients[i as usize][j as usize] = if j % 2 == 0 { 1 } else { -1 };
                }
            }
        }

        pub fn get_default_boundary_coefficients(&mut self, length: u32) -> Vec<i32> {
            if self.default_boundary_coefficients.len() == 0 {
                self.initialize_default_boundary_coefficients();
            }

            if (length < self.default_coefficient_cache_size) {
                return self.default_boundary_coefficients[length as usize].clone();
            }

            let mut boundary_coefficients: Vec<i32> = Vec::new();

            for i in 0..length as usize {
                boundary_coefficients[i] = if i % 2 == 0 { 1 } else { -1 };
            }

            return boundary_coefficients;
        }
    }
}