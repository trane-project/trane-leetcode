use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::greedy");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Greedy".to_string(),
        directory_name: "greedy".to_string(),
        easy_lesson: None,
        medium_lesson: Some(vec![
            LeetcodeExercise::new("maximum-subarray", "Maximum Subarray"),
            LeetcodeExercise::new("jump-game", "Jump Game"),
            LeetcodeExercise::new("jump-game-ii", "Jump Game II"),
            LeetcodeExercise::new("gas-station", "Gas Station"),
            LeetcodeExercise::new("hand-of-straights", "Hand of Straights"),
            LeetcodeExercise::new(
                "merge-triplets-to-form-target-triplet",
                "Merge Triplets to Form Target Triplet",
            ),
            LeetcodeExercise::new("partition-labels", "Partition Labels"),
            LeetcodeExercise::new("valid-parentheses-string", "Valid Parentheses String"),
        ]),
        hard_lesson: None,
    };
    course.course_builder()
}
