pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // TODO
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut input = vec![1, 1, 2];
        let expected = vec![1, 2, -1];
        let output = 2;
        let result = remove_duplicates(&mut input);
        assert_eq!(result, output);
        assert_eq!(input, expected);
    }
}
