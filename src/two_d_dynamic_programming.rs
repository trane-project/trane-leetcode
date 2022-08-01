use crate::{
    leetcode::{LeetcodeCourse, LeetcodeExercise},
    one_d_dynamic_programming,
};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::two_d_dynamic_programming");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![*one_d_dynamic_programming::COURSE_ID],
        topic: "2-D Dynamic Programming".to_string(),
        directory_name: "two_d_dynamic_programming".to_string(),
        easy_lesson: None,
        medium_lesson: Some(vec![
            LeetcodeExercise::new("unique-paths", "Unique Paths"),
            LeetcodeExercise::new("longest-common-subsequence", "Longest Common Subsequence"),
            LeetcodeExercise::new(
                "best-time-to-buy-and-sell-stock-with-cooldown",
                "Best Time to Buy and Sell Stock with Cooldown",
            ),
            LeetcodeExercise::new("coin-change-2", "Coin Change II"),
            LeetcodeExercise::new("target-sum", "Target Sum"),
            LeetcodeExercise::new("interleaving-string", "Interleaving String"),
        ]),
        hard_lesson: Some(vec![
            LeetcodeExercise::new(
                "longest-increasing-path-in-a-matrix",
                "Longest Increasing Path in a Matrix",
            ),
            LeetcodeExercise::new("distinct-subsequences", "Distinct Subsequences"),
            LeetcodeExercise::new("edit-distance", "Edit Distance"),
            LeetcodeExercise::new("burst-balloons", "Burst Balloons"),
            LeetcodeExercise::new("regular-expression-matching", "Regular Expression Matching"),
        ]),
    };
    course.course_builder()
}
