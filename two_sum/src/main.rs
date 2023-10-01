use std::collections::HashMap;

pub struct Solution {}

impl Solution {

    // Runtime 2ms (74,76% percentile), Memory 2.6MB (15.34% percentile)
    //
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut hashmap: HashMap<i32, usize> = HashMap::new();
    //     for (i, &num) in nums.iter().enumerate() {
    //         match hashmap.get(&(target - num)) {
    //             Some(&index) => return vec![index as i32,i as i32],
    //             None => hashmap.insert(num, i),
    //         };
    //     }
    //     unreachable!()
    // }

    // Runtime 0ms (100% percentile), Memory 2.4MB (29.22% percentile)
    //
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut hashmap: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    //     for (index, &number) in nums.iter().enumerate() {
    //         let expected_sum = target - number;
    //         match hashmap.get(&expected_sum) {
    //             Some(&previous_index) => return vec![previous_index as i32,index as i32],
    //             None => hashmap.insert(number, index),
    //         };
    //     }
    //     unreachable!()
    // }

    // Runtime 1ms (87.23% percentile), Memory 2.4MB (42.40% percentile)
    //
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for i in 0..nums.len() {
            let expected_sum = target - nums[i];
            match hashmap.get(&expected_sum) {
                Some(&previous_index) => return vec![previous_index as i32,i as i32],
                None => hashmap.insert(nums[i], i),
            };
        }
        unreachable!()
    }
}

fn main() {
    // Example 1
    let nums = vec![2,7,11,15];
    let target = 9;
    let output = vec![0,1];
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, output, "Example 1 does not match.");

    // Example 2
    let nums = vec![3,2,4];
    let target = 6;
    let output = vec![1,2];
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, output, "Example 2 does not match.");

    // Example 3
    let nums = vec![3,3];
    let target = 6;
    let output = vec![0,1];
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, output, "Example 3 does not match.");
}
