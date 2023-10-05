// https://leetcode.com/problems/first-missing-positive/description/

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    // TODO
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![1, 2, 0];
        let expected = 3;
        assert_eq!(first_missing_positive(input), expected);
    }

    #[test]
    fn example2() {
        let input = vec![3, 4, -1, 1];
        let expected = 2;
        assert_eq!(first_missing_positive(input), expected);
    }

    #[test]
    fn example3() {
        let input = vec![7, 8, 9, 11, 12];
        let expected = 1;
        assert_eq!(first_missing_positive(input), expected);
    }

    #[test]
    fn example4() {
        let input = vec![2, 7, 8, 9, 11, 12, 1];
        let expected = 3;
        assert_eq!(first_missing_positive(input), expected);
    }
}
