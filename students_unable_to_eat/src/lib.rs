use std::collections::VecDeque;

pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut student_queue = VecDeque::from(students);
    let mut sandwich_queue = VecDeque::from(sandwiches);
    while let Some(sandwich) = sandwich_queue.pop_front() {
        let mut student_count = student_queue.len();
        'find_student: while student_count > 0 {
            let student_preference = student_queue.pop_front().unwrap();
            if student_preference != sandwich {
                student_queue.push_back(student_preference);
                student_count -= 1;
            } else {
                break 'find_student;
            }
        }
        if student_count == 0 && !sandwich_queue.is_empty() {
            return student_queue.len() as i32;
        }
    }
    student_queue.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let students = vec![1, 1, 0, 0];
        let sandwiches = vec![0, 1, 0, 1];
        let expected = 0;
        assert_eq!(count_students(students, sandwiches), expected);
    }

    #[test]
    fn test2() {
        let students = vec![1, 1, 1, 0, 0, 1];
        let sandwiches = vec![1, 1, 1, 0, 0, 1];
        let expected = 0;
        assert_eq!(count_students(students, sandwiches), expected);
    }
}
