use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::backtracking");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Backtracking".to_string(),
        directory_name: "backtracking".to_string(),
        easy_lesson: None,
        medium_lesson: Some(vec![
            LeetcodeExercise::new("subsets", "Subsets"),
            LeetcodeExercise::new("combination-sum", "Combination Sum"),
            LeetcodeExercise::new("permutations", "Permutations"),
            LeetcodeExercise::new("subsets-ii", "Subsets II"),
            LeetcodeExercise::new("combination-sum-ii", "Combination Sum II"),
            LeetcodeExercise::new("word-search", "Word Search"),
            LeetcodeExercise::new("palindrome-partitioning", "Palindrome Partitioning"),
            LeetcodeExercise::new(
                "letter-combinations-of-a-phone-number",
                "Letter Combinations of a Phone Number",
            ),
        ]),
        hard_lesson: Some(vec![LeetcodeExercise::new("n-queens", "N-Queens")]),
    };
    course.course_builder()
}
