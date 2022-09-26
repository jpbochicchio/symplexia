pub mod intervals {
    use std::fmt::Debug;

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
        T: PartialEq + PartialOrd,
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

        pub fn contains_point(point: T) -> bool {
            todo!();
        }
    }

    impl<T> Debug for Interval<T>
    where
        T: PartialEq + PartialOrd + Debug + Clone + Default,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut left_interval_symbol: &str = "";
            let mut left_value_str: String = String::from("");
            let mut right_interval_symbol: &str = "";
            let mut right_value_str: String = String::from("");

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
                left_interval_symbol, &left_value_str, right_interval_symbol, &right_value_str
            );
        }
    }
}
