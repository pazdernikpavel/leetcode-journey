pub fn longest_common_prefix(words: Vec<String>) -> String {
    if words.is_empty() {
        return String::from("");
    }

    let mut common_prefix = String::clone(&words[0]);

    for word in &words[1..] {
        while !word.starts_with(&common_prefix) {
            common_prefix.truncate(common_prefix.len() - 1);
        }
    }

    common_prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        let expected = String::from("fl");
        assert_eq!(longest_common_prefix(input), expected);
    }

    #[test]
    fn example2() {
        let input = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];
        let expected = String::from("");
        assert_eq!(longest_common_prefix(input), expected);
    }
}
