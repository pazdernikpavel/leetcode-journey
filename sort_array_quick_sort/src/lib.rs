// Time Complexity: O(n log n) / Worst case n * n
// Space Complexity: O(1)

pub fn quick_sort(arr: &mut Vec<i32>, start: usize, end: usize) {
    if end - start < 1 {
        return;
    }

    let pivot = arr[end];
    let mut left = start;

    for i in start..end {
        if arr[i] < pivot {
            arr.swap(left, i);
            left += 1;
        }
    }

    arr.swap(left, end);
    quick_sort(arr, start, if left > start { left - 1 } else { start });
    quick_sort(arr, if left < end { left + 1 } else { end }, end);
}

pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    quick_sort(&mut nums, 0, len - 1);
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = sort_array(vec![2, 7, 4, 8, 10, 5, 1, 6, 3]);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 10]);
    }
    #[test]
    fn test2() {
        let result = sort_array(vec![5, 1, 1, 2, 0, 0]);
        assert_eq!(result, vec![0, 0, 1, 1, 2, 5]);
    }
}
