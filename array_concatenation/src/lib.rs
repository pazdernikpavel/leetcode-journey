// Time Complexity: O(n)
// Space Complexity: O(n)

pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec![0; nums.len() * 2];
    for i in 0..nums.len() {
        new_vec[i] = nums[i];
        new_vec[i + nums.len()] = nums[i];
    }
    new_vec
}

#[cfg(test)]
mod tests {
    use super::get_concatenation;

    #[test]
    fn test1() {
        let input = vec![1, 2, 1];
        let expected = vec![1, 2, 1, 1, 2, 1];
        let result = get_concatenation(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let input = vec![1, 3, 2, 1];
        let expected = vec![1, 3, 2, 1, 1, 3, 2, 1];
        let result = get_concatenation(input);
        assert_eq!(result, expected);
    }
}
