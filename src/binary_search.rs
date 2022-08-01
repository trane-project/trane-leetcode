use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::binary_search");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Binary Search".to_string(),
        directory_name: "binary_search".to_string(),
        easy_lesson: Some(vec![LeetcodeExercise::new(
            "binary-search",
            "Binary Search",
        )]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new("search-a-2d-matrix", "Search a 2D Matrix"),
            LeetcodeExercise::new("koko-eating-bananas", "Koko Eating Bananas"),
            LeetcodeExercise::new(
                "search-in-rotated-sorted-array",
                "Search Rotated Sorted Array",
            ),
            LeetcodeExercise::new(
                "find-minimum-in-rotated-sorted-array",
                "Find Minimum in Rotated Sorted Array",
            ),
            LeetcodeExercise::new("time-based-key-value-store", "Time Based Key-Value Store"),
        ]),
        hard_lesson: Some(vec![LeetcodeExercise::new(
            "median-of-two-sorted-arrays",
            "Median of Two Sorted Arrays",
        )]),
    };
    course.course_builder()
}
