pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut count = [0; 3];
    for (i, _) in nums.iter().enumerate() {
        let index = nums[i] as usize;
        count[index] += 1;
    }

    let mut i = 0usize;
    for (j, k) in count.iter().enumerate() {
        for _ in 0..*k {
            nums[i] = j as i32;
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut input = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut input);
        assert_eq!(input, vec![0, 0, 1, 1, 2, 2]);
    }
}
