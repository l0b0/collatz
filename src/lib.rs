pub fn next(current: isize) -> isize {
    current / 2
}

#[cfg(test)]
mod tests {
    use super::next;

    #[test]
    fn next_should_return_half_of_input_for_even_values() {
        assert_eq!(1, next(2));
        assert_eq!(2, next(4));
    }
}
