use practice::problems::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_find_missing_letter() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), Some('e'));
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'e', 'f']), None);
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'D', 'f']), Some('e'));
        assert_eq!(find_missing_letter(&[]), None);
    }

    #[test]
    #[ignore]
    fn test_generate_prime_array() {
        assert_eq!(generate_prime_array(1, 10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(&[1, 3, 4, 5], 5), Some((0, 2)));
    }
}
