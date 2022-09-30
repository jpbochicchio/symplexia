pub mod primitives {
    pub trait PrimitiveBasisElement {
        fn get_dimension(self) -> i32;
        fn get_boundary_array(self) -> Vec<Self> where Self: Sized;
        fn get_boundary_coefficients(self) -> Vec<i32>; 
    }
}