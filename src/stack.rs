use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::stack");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Stack".to_string(),
        directory_name: "stack".to_string(),
        easy_lesson: Some(vec![LeetcodeExercise::new(
            "valid-parentheses",
            "Valid Parentheses",
        )]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new("min-stack", "Min Stack"),
            LeetcodeExercise::new(
                "evaluate-reverse-polish-notation",
                "Evaluate Reverse Polish Notation",
            ),
            LeetcodeExercise::new("generate-parentheses", "Generate Parentheses"),
            LeetcodeExercise::new("daily-temperatures", "Daily Temperatures"),
            LeetcodeExercise::new("car-fleet", "Car Fleet"),
        ]),
        hard_lesson: Some(vec![LeetcodeExercise::new(
            "longest-rectangle-in-histogram",
            "Longest Rectangle in Histogram",
        )]),
    };
    course.course_builder()
}
