use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::arrays_hashing");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Arrays & Hashing".to_string(),
        directory_name: "arrays_hashing".to_string(),
        easy_lesson: Some(vec![
            LeetcodeExercise::new("contains-duplicate", "Contains Duplicate"),
            LeetcodeExercise::new("valid-anagram", "Valid Anagram"),
            LeetcodeExercise::new("two-sum", "Two Sum"),
        ]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new("group-anagrams", "Group Anagrams"),
            LeetcodeExercise::new("top-k-frequent-elements", "Top K Frequent Elements"),
            LeetcodeExercise::new(
                "product-of-array-except-self",
                "Product of Array Except Self",
            ),
            LeetcodeExercise::new("valid-sudoky", "Valid Sudoku"),
            LeetcodeExercise::new("encode-and-decode-strings", "Encode and Decode Strings"),
            LeetcodeExercise::new(
                "longest-consecutive-sequence",
                "Longest Consecutive Sequence",
            ),
        ]),
        hard_lesson: None,
    };
    course.course_builder()
}
