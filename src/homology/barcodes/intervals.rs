#![allow(dead_code)]
pub mod intervals {
    use core::cmp::Ordering::*;
    use std::{
        fmt::{Debug, Display},
        ops::Sub,
    };

    const UID: i64 = -3434702188482864510;

    #[derive(Clone, PartialEq)]
    pub struct Interval<T> {
        serial_version_uid: i64,
        start: Option<T>,
        end: Option<T>,
        is_left_closed: bool,
        is_right_closed: bool,
        is_left_infinite: bool,
        is_right_infinite: bool,
    }

    impl<T> Interval<T>
    where
        T: PartialEq + PartialOrd + Clone + Debug,
    {
        pub fn new(
            start: Option<T>,
            end: Option<T>,
            is_left_closed: bool,
            is_right_closed: bool,
            is_left_infinite: bool,
            is_right_infinite: bool,
        ) -> Self {
            Interval {
                serial_version_uid: UID,
                start: start,
                end: end,
                is_left_closed,
                is_right_closed,
                is_left_infinite,
                is_right_infinite,
            }
        }

        pub fn finite_closed_interval(start: T, end: T) -> Self {
            Interval::new(Some(start), Some(end), true, true, false, false)
        }

        pub fn finite_right_open_interval(start: T, end: T) -> Self {
            Interval::new(Some(start), Some(end), true, false, false, false)
        }

        pub fn finite_left_open_interval(start: T, end: T) -> Self {
            Interval::new(Some(start), Some(end), false, true, false, false)
        }

        pub fn finite_open_interval(start: T, end: T) -> Self {
            Interval::new(Some(start), Some(end), false, false, false, false)
        }

        pub fn right_infinite_closed_interval(start: T) -> Self {
            Interval::new(Some(start), None::<T>, true, true, false, true)
        }

        pub fn right_infinite_open_interval(start: T) -> Self {
            Interval::new(Some(start), None::<T>, false, false, false, true)
        }

        pub fn right_infinite_right_open_interval(start: T) -> Self {
            Interval::new(Some(start), None::<T>, true, false, false, true)
        }

        pub fn right_infinite_left_open_interval(start: T) -> Self {
            Interval::new(Some(start), None::<T>, false, true, false, true)
        }

        pub fn left_infinite_closed_interval(end: T) -> Self {
            Interval::new(None::<T>, Some(end), true, true, true, false)
        }

        pub fn left_infinite_right_open_interval(end: T) -> Self {
            Interval::new(None::<T>, Some(end), true, false, true, false)
        }

        pub fn left_infinite_left_open_interval(end: T) -> Self {
            Interval::new(None::<T>, Some(end), false, false, true, false)
        }

        pub fn left_infinite_open_interval(end: T) -> Self {
            Interval::new(None::<T>, Some(end), false, false, true, false)
        }

        pub fn is_infinite(self) -> bool {
            self.is_left_infinite || self.is_right_infinite
        }

        pub fn get_start(self) -> Option<T> {
            return self.start;
        }

        pub fn get_end(self) -> Option<T> {
            return self.end;
        }

        pub fn contains_point(&self, point: T) -> bool {
            if !self.is_left_infinite {
                if self.is_left_closed && &point < self.start.as_ref().unwrap_or(&point) {
                    return false;
                } else if !self.is_left_closed && &point <= self.start.as_ref().unwrap_or(&point) {
                    return false;
                }
            }

            if !self.is_right_infinite {
                if self.is_right_closed && &point > self.end.as_ref().unwrap_or(&point) {
                    return false;
                } else if !self.is_right_closed && &point >= self.end.as_ref().unwrap_or(&point) {
                    return false;
                }
            }

            return true;
        }
    }

    impl<T> Debug for Interval<T>
    where
        T: PartialEq + PartialOrd + Debug + Clone,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            return write!(f, "{}", self.to_string());
        }
    }

    impl<T> PartialOrd for Interval<T>
    where
        T: PartialEq + PartialOrd + Sub<Output = T> + Clone + Debug,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            // Infinite cases
            if self.clone().is_infinite() && other.clone().is_infinite() {
                return Some(Equal);
            }

            if self.clone().is_infinite() {
                return Some(Greater);
            }

            if other.clone().is_infinite() {
                return Some(Less);
            }

            // Finite cases
            let l_0: T = self.end.clone().unwrap() - self.start.clone().unwrap();
            let l_1: T = other.end.clone().unwrap() - other.start.clone().unwrap();

            return l_0.partial_cmp(&l_1);
        }
    }

    impl<T> Display for Interval<T>
    where
        T: Clone + Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let left_interval_symbol: &str;
            let left_value_str: String;
            let right_interval_symbol: &str;
            let right_value_str: String;

            if self.is_left_closed {
                left_interval_symbol = "[";
            } else {
                left_interval_symbol = "(";
            }

            if self.is_left_infinite {
                left_value_str = String::from("-infinity");
            } else {
                left_value_str = format!("{:?}", self.start.clone().unwrap());
            }

            if self.is_right_closed {
                right_interval_symbol = "]";
            } else {
                right_interval_symbol = ")";
            }

            if self.is_right_infinite {
                right_value_str = String::from("infinity");
            } else {
                right_value_str = format!("{:?}", self.end.clone().unwrap());
            }

            return write!(
                f,
                "{}{}, {}{}",
                left_interval_symbol, left_value_str, right_value_str, right_interval_symbol
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::intervals::Interval;
    use core::cmp::Ordering::*;

    #[test]
    fn test_interval_printing() {
        let finite_left_open_interval: Interval<i32> = Interval::finite_left_open_interval(2, 5);
        let finite_right_open_interval: Interval<i32> = Interval::finite_right_open_interval(3, 6);
        let finite_closed_interval: Interval<i32> = Interval::finite_closed_interval(-2, 3);
        let finite_open_interval: Interval<i32> = Interval::finite_open_interval(-10, 10);

        let left_infinite_open_interval: Interval<i32> = Interval::left_infinite_open_interval(10);
        let right_infinite_open_interval: Interval<i32> =
            Interval::right_infinite_open_interval(10);
        let left_infinite_closed_interval: Interval<i32> =
            Interval::left_infinite_closed_interval(10);
        let right_infinite_closed_interval: Interval<i32> =
            Interval::right_infinite_closed_interval(10);

        assert_eq!(
            finite_left_open_interval.to_string(),
            String::from("(2, 5]")
        );
        assert_eq!(
            finite_right_open_interval.to_string(),
            String::from("[3, 6)")
        );
        assert_eq!(finite_closed_interval.to_string(), String::from("[-2, 3]"));
        assert_eq!(finite_open_interval.to_string(), String::from("(-10, 10)"));

        assert_eq!(
            left_infinite_open_interval.to_string(),
            String::from("(-infinity, 10)")
        );
        assert_eq!(
            right_infinite_open_interval.to_string(),
            String::from("(10, infinity)")
        );
        assert_eq!(
            left_infinite_closed_interval.to_string(),
            String::from("[-infinity, 10]")
        );
        assert_eq!(
            right_infinite_closed_interval.to_string(),
            String::from("[10, infinity]")
        );
    }

    #[test]
    fn test_finite_partial_order() {
        let small_finite_interval: Interval<i16> = Interval::finite_open_interval(1, 3);
        let large_finite_interval: Interval<i16> = Interval::finite_open_interval(-100, 100);

        assert_eq!(
            small_finite_interval.partial_cmp(&large_finite_interval),
            Some(Less)
        );
        assert_eq!(
            large_finite_interval.partial_cmp(&small_finite_interval),
            Some(Greater)
        );
    }

    #[test]
    fn test_infinite_partial_order() {
        let small_infinite_interval: Interval<i8> = Interval::left_infinite_open_interval(1);
        let large_infinite_interval: Interval<i8> = Interval::right_infinite_closed_interval(100);

        assert_eq!(
            small_infinite_interval.partial_cmp(&large_infinite_interval),
            Some(Equal)
        );
        assert_eq!(
            large_infinite_interval.partial_cmp(&small_infinite_interval),
            Some(Equal)
        );
    }

    #[test]
    fn test_mixed_partial_order() {
        let finite_interval: Interval<i8> = Interval::finite_open_interval(1, 3);
        let infinite_interval: Interval<i8> = Interval::left_infinite_open_interval(1);

        assert_eq!(
            infinite_interval.partial_cmp(&finite_interval),
            Some(Greater)
        );
        assert_eq!(finite_interval.partial_cmp(&infinite_interval), Some(Less));
    }
}
