use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::bit_manipulation");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Bit Manipulation".to_string(),
        directory_name: "bit_manipulation".to_string(),
        easy_lesson: Some(vec![
            LeetcodeExercise::new("single-number", "Single Number"),
            LeetcodeExercise::new("number-of-1-bits", "Number of 1 Bits"),
            LeetcodeExercise::new("counting-bits", "Counting Bits"),
            LeetcodeExercise::new("reverse-bits", "Reverse Bits"),
            LeetcodeExercise::new("missing-number", "Missing Number"),
        ]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new("sum-of-two-integers", "Sum of Two Integers"),
            LeetcodeExercise::new("reverse-integer", "Reverse Integer"),
        ]),
        hard_lesson: None,
    };
    course.course_builder()
}
