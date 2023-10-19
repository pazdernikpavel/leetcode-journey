pub fn climb_stairs(n: i64) -> i64 {
    if n <= 2 {
        n
    } else {
        (0..n + 1)
            .fold((0, 1), |(left, right), _| (right, left + right))
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(climb_stairs(45), 1836311903);
    }
}
