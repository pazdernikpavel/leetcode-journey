// Time Complexity: O(log n)
// Space Complexity: O(1)

struct Solution {}
impl Solution {
    fn isBadVersion(version: i32) -> bool {
        if version >= 2 {
            true
        } else {
            false
        }
    }
    pub fn first_bad_version(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left <= right {
            let mid = left + ((right - left) >> 1);
            if Solution::isBadVersion(mid) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        right + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::first_bad_version(5), 4);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::first_bad_version(1), 1);
    }
    #[test]
    fn test3() {
        assert_eq!(Solution::first_bad_version(3), 2);
    }
}
