// Time Complexity: O(n log n)
// Space Complexity: O(n)

pub fn merge_sort(arr: &mut Vec<i32>, start: usize, end: usize) {
    if end - start < 1 {
        return;
    }

    let middle = (start + end) / 2;
    merge_sort(arr, start, middle);
    merge_sort(arr, middle + 1, end);
    merge(arr, start, middle, end);
}

pub fn merge(arr: &mut [i32], start: usize, middle: usize, end: usize) {
    let left = arr[start..=middle].to_vec();
    let right = arr[middle + 1..end + 1].to_vec();

    let mut k = start;
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] < right[right_index] {
            arr[k] = left[left_index];
            left_index += 1;
        } else {
            arr[k] = right[right_index];
            right_index += 1;
        }
        k += 1;
    }

    while left_index < left.len() {
        arr[k] = left[left_index];
        left_index += 1;
        k += 1;
    }

    while right_index < right.len() {
        arr[k] = right[right_index];
        right_index += 1;
        k += 1;
    }
}

pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    merge_sort(&mut nums, 0, len - 1);
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
}
