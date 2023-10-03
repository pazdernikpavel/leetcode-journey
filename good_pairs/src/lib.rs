// Given an array of integers nums, return the number of good pairs.
// A pair (i, j) is called good if nums[i] == nums[j] and i < j.

// Time Complexity: O(n2)
// Space Complexity: O(n)

// pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
//     let mut hashmap: HashMap<i32, Vec<usize>> = HashMap::with_capacity(nums.len());
//     let mut good_pairs = 0;
//     for (index, value) in nums.iter().enumerate() {
//         match hashmap.get_mut(value) {
//             Some(existing_indexes) => {
//                 good_pairs += existing_indexes
//                     .iter()
//                     .fold(0, | total_sum, &current_index | {
//                         if index > current_index {
//                             total_sum + 1
//                         } else {
//                             total_sum
//                         }
//                     });
//                 existing_indexes.push(index);
//             },
//             None => {
//                 hashmap.insert(*value, vec![index]);
//             }
//         }
//     }
//     good_pairs
// }

// Time Complexity: O(n2)
// Space Complexity: O(1)

// pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
//     let mut good_pairs = 0;
//     for (i, value_a) in nums.iter().enumerate() {
//         for value_b in nums[i+1..].iter() {
//             if *value_a == *value_b {
//                 good_pairs += 1;
//             }
//         }
//     }
//     good_pairs
// }

use std::collections::{HashMap};

// Time Complexity: O(n)
// Space Complexity: O(n)

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut hashset: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    let mut good_pairs = 0;
    for value in nums.iter() {
        match hashset.get(value) {
            Some(occurrences) => {
                good_pairs += occurrences;
                hashset.insert(*value, 1 + occurrences);
            }
            _ => { hashset.insert(*value, 1); }
        }
    }
    good_pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![1,2,3,1,1,3];
        assert_eq!(num_identical_pairs(input), 4);
    }

    #[test]
    fn example2() {
        let input = vec![1,1,1,1];
        assert_eq!(num_identical_pairs(input), 6);
    }

    #[test]
    fn example3() {
        let input = vec![1,2,3];
        assert_eq!(num_identical_pairs(input), 0);
    }
}
