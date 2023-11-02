use std::cmp::Ordering;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    for row in matrix {
        if target >= row[0] && target <= row[row.len() - 1] {
            let mut left = 0;
            let mut right = row.len() - 1;
            while left <= right {
                let mid = (left + right) / 2;
                match target.cmp(&row[mid]) {
                    Ordering::Greater => {
                        left = mid + 1;
                    }
                    Ordering::Less => {
                        right = mid - 1;
                    }
                    Ordering::Equal => return true,
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let input = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;

        assert!(search_matrix(input, target));
    }

    #[test]
    fn test2() {
        let input = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        assert!(!search_matrix(input, target));
    }
}
