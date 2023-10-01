// Runtime 0ms (100% percentile), Memory 2.3MB (33.33% percentile)
pub fn reverse_words(s: String) -> String {
    s.split(" ")
        .map(|x| x.chars()
            .rev()
            .collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = String::from("Let's take LeetCode contest");
        let expected = String::from("s'teL ekat edoCteeL tsetnoc");
        assert_eq!(reverse_words(input), expected);
    }

    #[test]
    fn example_2() {
        let input = String::from("God Ding");
        let expected = String::from("doG gniD");
        assert_eq!(reverse_words(input), expected);
    }
}
