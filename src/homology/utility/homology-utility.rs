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

        pub fn remove_index(arr: Vec<i32>, index: u32) -> Vec<i32> {
            let mut res: Vec<i32> = Vec::new();
            let (mut new_index, mut old_index) = (0 as u32, 0 as u32);

            while old_index < arr.len() as u32 {
                if old_index == index { 
                    old_index += 1; 
                } else {
                    res[new_index as usize] = *arr.get(old_index as usize).unwrap();
                    new_index += 1;
                    old_index += 1;
                }
            }

            return res;
        }
    }
}