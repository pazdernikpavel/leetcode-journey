// Brute force recursion
//
// pub fn fib(n: i32) -> i32 {
//     if n <= 1 {
//         n
//     } else {
//         fib(n - 1) + fib(n - 2)
//     }
// }

pub fn fib(n: i32) -> i32 {
    (0..n)
        .fold((0, 1), |(left, right), _| (right, left + right))
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(fib(3), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(fib(4), 3);
    }
}
