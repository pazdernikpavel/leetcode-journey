pub fn roman_to_int(s: String) -> i32 {
    s.chars().fold([0; 4], |[m, c, x, i], ch| match ch {
        'M' => [m + 1000 - c, 0, x, i],
        'D' => [m + 500 - c, 0, x, i],
        'C' => [m, c + 100 - x, 0, i],
        'L' => [m, c + 50 - x, 0, i],
        'X' => [m, c, x + 10 - i, 0],
        'V' => [m, c, x + 5 - i, 0],
        _ => [m, c, x, i + 1],
    }).into_iter().sum::<i32>()
}

mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = String::from("III");
        let expected = 3;
        assert_eq!(roman_to_int(input), expected);
    }

    #[test]
    fn example_2() {
        let input = String::from("LVIII");
        let expected = 58;
        assert_eq!(roman_to_int(input), expected);
    }

    #[test]
    fn example_3() {
        let input = String::from("MCMXCIV");
        let expected = 1994;
        assert_eq!(roman_to_int(input), expected);
    }
}
