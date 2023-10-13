// Time Complexity: O(n)
// Space Complexity: O(1)

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    match nums.is_empty() {
        true => 0,
        false => {
            let mut prev = 0;
            for i in 1..nums.len() {
                if nums[prev] != nums[i] {
                    prev += 1;
                    nums[prev] = nums[i];
                }
            }
            (prev + 1) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut input = vec![1, 1, 2];
        let expected = vec![1, 2, 2];
        let output = 2;
        let result = remove_duplicates(&mut input);
        assert_eq!(result, output);
        assert_eq!(input, expected);
    }

    #[test]
    fn test2() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected = vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4];
        let output = 5;
        let result = remove_duplicates(&mut input);
        assert_eq!(result, output);
        assert_eq!(input, expected);
    }
}
