// https://leetcode.com/problems/median-of-two-sorted-arrays/

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let first = vec![1, 3];
        let second = vec![2];
        assert_eq!(Solution::find_median_sorted_arrays(first, second), 2f64);
    }
}
