use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::one_d_dynamic_programming");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "1-D Dynamic Programming".to_string(),
        directory_name: "one_d_dynamic_programming".to_string(),
        easy_lesson: Some(vec![
            LeetcodeExercise::new("climbing-stairs", "Climbing Stairs"),
            LeetcodeExercise::new("Min Cost Climbing Stairs", "Min Cost Climbing Stairs"),
        ]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new("house-robber", "House Robber"),
            LeetcodeExercise::new("house-robber-ii", "House Robber II"),
            LeetcodeExercise::new(
                "longest-palindrome-substring",
                "Longest Palindrome Substring",
            ),
            LeetcodeExercise::new("palindromic-substrings", "Palindromic Substrings"),
            LeetcodeExercise::new("decode-ways", "Decode Ways"),
            LeetcodeExercise::new("coin-change", "Coin Change"),
            LeetcodeExercise::new("maximum-product-subarray", "Maximum Product Subarray"),
            LeetcodeExercise::new("word-break", "Word Break"),
            LeetcodeExercise::new(
                "longest-increasing-subsequence",
                "Longest Increasing Subsequence",
            ),
            LeetcodeExercise::new("partition-equal-subset-sum", "Partition Equal Subset Sum"),
        ]),
        hard_lesson: None,
    };
    course.course_builder()
}
