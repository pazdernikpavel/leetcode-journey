// Time Complexity: O(n)
// Space Complexity: O(1)

// Using built-in filter function
//
// pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
//     nums.retain(|&element| element != val);
//     nums.len() as i32
// }

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    match nums.len() {
        0 => 0,
        len => {
            let mut prev = 0;
            for i in 0..len {
                if nums[i] != val {
                    nums[prev] = nums[i];
                    prev += 1;
                }
            }
            prev as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut input = vec![3, 2, 2, 3];
        let expected = vec![2, 2, 2, 3];
        let output = 2;
        let result = remove_element(&mut input, 3);
        assert_eq!(result, output);
        assert_eq!(input, expected);
    }
}
