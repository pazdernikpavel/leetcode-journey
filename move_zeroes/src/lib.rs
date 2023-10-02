pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut j: usize = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, j);
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut input = vec![0, 1, 0, 3, 12];
        let expected = vec![1, 3, 12, 0, 0];
        move_zeroes(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn example2() {
        let mut input = vec![0];
        let expected = vec![0];
        move_zeroes(&mut input);
        assert_eq!(input, expected);
    }
}
