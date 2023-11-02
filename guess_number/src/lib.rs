use std::cmp::Ordering;

unsafe fn guess(num: i32) -> i32 {
    let picked = 6;
    match picked.cmp(&num) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    }
}

unsafe fn guessNumber(n: i32) -> i32 {
    let mut left = 1;
    let mut right = n;
    while left <= right {
        let mid = (left + right) / 2;
        match guess(mid) {
            1 => {
                left = mid + 1;
            }
            -1 => {
                right = mid - 1;
            }
            0 => return mid,
            _ => {}
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        unsafe {
            assert_eq!(guessNumber(10), 6);
        }
    }
}
