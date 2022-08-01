use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::two_pointers");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Two Pointers".to_string(),
        directory_name: "two_pointers".to_string(),
        easy_lesson: Some(vec![LeetcodeExercise::new(
            "valid-palindrome",
            "Valid Palindrome",
        )]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new("two-sum-ii-input-array-is-sorted", "Two Sum II"),
            LeetcodeExercise::new("3sum", "3Sum"),
            LeetcodeExercise::new("container-with-most-water", "Container With Most Water"),
        ]),
        hard_lesson: Some(vec![LeetcodeExercise::new(
            "trapping-rain-water",
            "Trapping Rain Water",
        )]),
    };
    course.course_builder()
}
