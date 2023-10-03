// You're given strings jewels representing the types of stones that are jewels,
// and stones representing the stones you have. Each character in stones is a type of stone
// you have. You want to know how many of the stones you have are also jewels.
//
// Letters are case sensitive, so "a" is considered a different type of stone from "A".

// Constraints
// 1 <= jewels.length, stones.length <= 50
// jewels and stones consist of only English letters.
// All the characters of jewels are unique.

use std::collections::HashSet;

// Time Complexity: O(J + S)
// Space Complexity: O(J)

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let hashset: HashSet<char> = jewels.chars().collect();
    stones.chars().filter(|&character| hashset.contains(&character)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let jewels = String::from("aA");
        let stones = String::from("aAAbbbb");
        let expected_output = 3;
        assert_eq!(num_jewels_in_stones(jewels, stones), expected_output);
    }

    #[test]
    fn example2() {
        let jewels = String::from("z");
        let stones = String::from("ZZ");
        let expected_output = 0;
        assert_eq!(num_jewels_in_stones(jewels, stones), expected_output);
    }

}
