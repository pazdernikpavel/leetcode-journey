// TODO: https://leetcode.com/problems/design-browser-history/

struct BrowserHistory {}

impl BrowserHistory {
    fn new(homepage: String) -> Self {}

    fn visit(&self, url: String) {}

    fn back(&self, steps: i32) -> String {}

    fn forward(&self, steps: i32) -> String {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let mut history = BrowserHistory::new(String::from("leetcode.com"));
        history.visit(String::from("google.com"));
        history.visit(String::from("facebook.com"));
        history.visit(String::from("youtube.com"));
        assert_eq!(history.back(1), String::from("facebook.com"));
        assert_eq!(history.back(1), String::from("google.com"));
        assert_eq!(history.forward(1), String::from("facebook.com"));
        history.visit(String::from("linkedin.com"));
        history.forward(2);
        assert_eq!(history.forward(2), String::from("google.com"));
        assert_eq!(history.forward(7), String::from("leetcode.com"));
    }
}
