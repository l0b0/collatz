pub fn next(current: isize) -> isize {
    if current & 1 == 0 {
        current / 2
    } else {
        1 + current * 3
    }
}

#[cfg(test)]
mod tests {
    use super::next;

    #[test]
    fn next_should_return_half_of_input_for_even_values() {
        assert_eq!(1, next(2));
        assert_eq!(2, next(4));
    }

    #[test]
    fn next_should_return_one_plus_three_times_input_for_odd_values() {
        assert_eq!(10, next(3));
        assert_eq!(16, next(5));
    }
}
