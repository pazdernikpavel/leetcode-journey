pub fn is_valid(content: String) -> bool {
    if content.len() & 1 == 1 {
        return false;
    }
    let mut stack = Vec::with_capacity(content.len());
    for c in content.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            _ => match stack.pop() {
                Some('(') if c == ')' => (),
                Some('[') if c == ']' => (),
                Some('{') if c == '}' => (),
                _ => return false,
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = String::from("()");
        assert!(is_valid(input));
    }

    #[test]
    fn test2() {
        let input = String::from("()[]{}");
        assert!(is_valid(input));
    }

    #[test]
    fn test3() {
        let input = String::from("(]");
        assert!(!is_valid(input));
    }
}
