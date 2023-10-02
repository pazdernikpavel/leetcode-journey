use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut visited_values: HashSet<i32> = HashSet::with_capacity(nums.len());
    for value in nums {
        match visited_values.get(&value) {
            Some(_) => return true,
            None => {
                visited_values.insert(value);
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![1, 2, 3, 1];
        assert!(contains_duplicate(input));
    }

    #[test]
    fn example2() {
        let input = vec![1, 2, 3, 4];
        assert!(!contains_duplicate(input));
    }

    #[test]
    fn example3() {
        let input = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert!(contains_duplicate(input));
    }
}
