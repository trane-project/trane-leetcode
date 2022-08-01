use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::tries");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Tries".to_string(),
        directory_name: "tries".to_string(),
        easy_lesson: None,
        medium_lesson: Some(vec![
            LeetcodeExercise::new("implement-trie-prefix-tree", "Implement Trie (Prefix Tree)"),
            LeetcodeExercise::new(
                "design-add-and-search-words-data-structure",
                "Design Add and Search Words Data Structure",
            ),
        ]),
        hard_lesson: Some(vec![LeetcodeExercise::new(
            "word-search-ii",
            "Word Search II",
        )]),
    };
    course.course_builder()
}
