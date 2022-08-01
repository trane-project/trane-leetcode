use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::linked_list");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Linked List".to_string(),
        directory_name: "linked_list".to_string(),
        easy_lesson: Some(vec![
            LeetcodeExercise::new("reverse-linked-list", "Reverse Linked List"),
            LeetcodeExercise::new("merge-two-sorted-lists", "Merge Two Sorted Lists"),
            LeetcodeExercise::new("linked-list-cycle", "Linked List Cycle"),
        ]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new("reorder-list", "Reorder List"),
            LeetcodeExercise::new(
                "remove-nth-node-from-end-of-list",
                "Remove Nth Node From End of List",
            ),
            LeetcodeExercise::new(
                "copy-list-with-random-pointer",
                "Copy List with Random Pointer",
            ),
            LeetcodeExercise::new("add-two-numbers", "Add Two Numbers"),
            LeetcodeExercise::new("find-the-duplicate-number", "Find the Duplicate Number"),
            LeetcodeExercise::new("lru-cache", "LRU Cache"),
        ]),
        hard_lesson: Some(vec![
            LeetcodeExercise::new("merge-k-sorted-lists", "Merge k Sorted Lists"),
            LeetcodeExercise::new("reverse-node-in-k-group", "Reverse Node in k Group"),
        ]),
    };
    course.course_builder()
}
