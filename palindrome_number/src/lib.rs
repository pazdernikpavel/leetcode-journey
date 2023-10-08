// https://leetcode.com/problems/palindrome-number/

// Using string, bad perf ❌

// pub fn is_palindrome(x: i32) -> bool {
//     let original_str = x.to_string();
//     let reversed_str: String = original_str.chars().rev().collect();
//     original_str == reversed_str
// }

pub fn is_palindrome(x: i32) -> bool {
    let mut copy = x;
    let mut reversed = 0;
    while copy > 0 {
        let digit = copy % 10;
        reversed = reversed * 10 + digit;
        copy /= 10;
    }
    x == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = 121;
        assert_eq!(is_palindrome(input), true);
    }

    #[test]
    fn example2() {
        let input = -121;
        assert!(!is_palindrome(input));
    }

    #[test]
    fn example3() {
        let input = 10;
        assert!(!is_palindrome(input));
    }

    #[test]
    fn example4() {
        let input = 1001;
        assert!(!is_palindrome(input));
    }
}
