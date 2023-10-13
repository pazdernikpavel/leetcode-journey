pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];
    for val in operations {
        match val.as_str() {
            "C" => {
                stack.pop();
            }
            "D" => {
                stack.push(2 * stack[stack.len() - 1]);
            }
            "+" => {
                stack.push(stack[stack.len() - 1] + stack[stack.len() - 2]);
            }
            string_val => {
                let res: Result<i32, _> = string_val.parse();
                if let Ok(parsed_val) = res {
                    stack.push(parsed_val);
                }
            }
        }
    }
    stack.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::cal_points;

    #[test]
    fn test1() {
        let input = vec![
            String::from("5"),
            String::from("2"),
            String::from("C"),
            String::from("D"),
            String::from("+"),
        ];
        let expected = 30;
        let output = cal_points(input);
        assert_eq!(output, expected);
    }
}
