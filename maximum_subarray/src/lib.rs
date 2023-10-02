use std::cmp::max;

// Kadane's algo => https://medium.com/@rsinghal757/kadanes-algorithm-dynamic-programming-how-and-why-does-it-work-3fd8849ed73d

// pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//     let mut maximum_so_far = nums[0];
//     let mut maximum_ending_here = nums[0];
//     for i in 1..nums.len() {
//         maximum_ending_here = max(nums[i], maximum_ending_here + nums[i]);
//         maximum_so_far = max(maximum_so_far, maximum_ending_here);
//     }
//     maximum_so_far
// }

// pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//     nums.iter().fold((i32::MIN, 0), |(max_sum, cur_sum), &num| {
//         let cur_sum = max(cur_sum + num, num);
//         (max(max_sum, cur_sum), cur_sum)
//     }).0
// }

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    let mut cur_sum = 0;

    for &x in nums.iter() {
        cur_sum = max(cur_sum + x, x);
        max_sum = max(max_sum, cur_sum)
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![-2,1,-3,4,-1,2,1,-5,4];
        let expected = 6;
        assert_eq!(max_sub_array(input), expected);
    }

    #[test]
    fn example2() {
        let input = vec![1];
        let expected = 1;
        assert_eq!(max_sub_array(input), expected);
    }

    #[test]
    fn example3() {
        let input = vec![5,4,-1,7,8];
        let expected = 23;
        assert_eq!(max_sub_array(input), expected);
    }

}
