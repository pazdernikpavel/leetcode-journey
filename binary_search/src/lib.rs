use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if target < nums[0] || target > nums[nums.len() - 1] {
        return -1;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        match target.cmp(&nums[mid]) {
            Ordering::Greater => {
                left = mid + 1;
            }
            Ordering::Less => {
                right = mid - 1;
            }
            Ordering::Equal => return mid as i32,
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let expected = 4;
        assert_eq!(search(input, target), expected);
    }

    #[test]
    fn test2() {
        let input = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let expected = -1;
        assert_eq!(search(input, target), expected);
    }
}
