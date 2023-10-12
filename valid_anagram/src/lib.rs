pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut map = std::collections::HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    println!("{:?}", map);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
    println!("{:?}", map);
    !map.into_values().any(|count| count != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = String::from("anagram");
        let b = String::from("nagaram");
        assert!(is_anagram(a, b));
    }

    #[test]
    fn test2() {
        let a = String::from("rat");
        let b = String::from("car");
        assert!(!is_anagram(a, b));
    }
}
