use practice::problems::*;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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
    #[ignore]
    fn test_two_sum() {
        assert_eq!(two_sum(&[1, 3, 4, 5], 5), Some((0, 2)));
        assert_eq!(two_sum(&[1, 1, 1,], 5), None);
        assert_eq!(two_sum(&[5, 5, 0], 5), Some((1, 2))); // 0, 2
        assert_eq!(two_sum(&[3, 1, 0, 4, 5], 5), Some((1, 3)));
    }

    #[test]
    #[ignore]
    fn test_organize() {
        let input = "Alice,30,Engineer,Bob,25,Designer";
        let expected = vec![
            Person {
                name: "Alice".to_string(),
                age: 30,
                occupation: "Engineer".to_string(),
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
                occupation: "Designer".to_string(),
            },
        ];
        assert_eq!(organize(input), expected);
    }

    #[test]
    #[ignore]
    fn test_letter_indices() {
        let mut expected = HashMap::new();
        expected.insert('h', vec![0]);
        expected.insert('e', vec![1]);
        expected.insert('l', vec![2, 3]);
        expected.insert('o', vec![4]);
        assert_eq!(letter_indices("hello"), expected);
    }

    #[test]
    #[ignore]
    fn test_letter_changes() {
        assert_eq!(letter_changes("abcXYZ"), "defABC");
        assert_eq!(letter_changes("Hello, World!"), "Khoor, Zruog!");
    }

    #[test]
    #[ignore]
    fn test_min_subarray_len() {
        assert_eq!(min_subarray_len(&[2, 3, 1, 2, 4, 3], 7), 2);
        assert_eq!(min_subarray_len(&[1, 4, 4], 4), 1);
        assert_eq!(min_subarray_len(&[2, 3, 5, 4, 2], 7), 2);
        assert_eq!(min_subarray_len(&[1, 1, 1, 1, 1, 1, 1, 1], 11), 0);
    }

    #[test]
    fn test_sort_odd() {
        let mut v1 = [5, 3, 2, 8, 1, 4];
        sort_odd(&mut v1);
        assert_eq!(v1, [1, 3, 2, 8, 5, 4]);

        let mut v2: [i32; 0] = [];
        sort_odd(&mut v2);
        assert_eq!(v2, []);

        let mut v3 = [7];
        sort_odd(&mut v3);
        assert_eq!(v3, [7]);

        let mut v4 = [4, 2, 8, 6];
        sort_odd(&mut v4);
        assert_eq!(v4, [4, 2, 8, 6]);

        let mut v5 = [9, 3, 5, 1];
        sort_odd(&mut v5);
        assert_eq!(v5, [1, 3, 5, 9]);

        let mut v6 = [9, 3, 9, 1, 3];
        sort_odd(&mut v6);
        assert_eq!(v6, [1, 3, 3, 9, 9]);

        let mut v7 = [5, -3, 2, -1];
        sort_odd(&mut v7);
        assert_eq!(v7, [-3, -1, 2, 5]);
    }
}
