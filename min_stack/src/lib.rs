// TODO: https://leetcode.com/problems/min-stack/

struct MinStack {}

impl MinStack {
    fn new() -> Self {}
    fn push(&self, val: i32) {}
    fn pop(&self) {}
    fn top(&self) -> i32 {}
    fn get_min(&self) -> i32 {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
