use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::sliding_window");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Sliding Window".to_string(),
        directory_name: "sliding_window".to_string(),
        easy_lesson: Some(vec![LeetcodeExercise::new(
            "best-time-to-buy-and-sell-stock",
            "Best Time to Buy and Sell Stock",
        )]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new(
                "longest-substring-without-repeating-characters",
                "Longest Substring Without Repeating Characters",
            ),
            LeetcodeExercise::new(
                "longest-repeating-character-replacement",
                "Longest Repeating Character Replacement",
            ),
            LeetcodeExercise::new("permutation-in-string", "Permutation in String"),
        ]),
        hard_lesson: Some(vec![
            LeetcodeExercise::new("minimum-window-substring", "Minimum Window Substring"),
            LeetcodeExercise::new("sliding-window-maximum", "Sliding Window Maximum"),
        ]),
    };
    course.course_builder()
}
