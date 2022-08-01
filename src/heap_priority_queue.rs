use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::heap_priority_queue");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Heap / Priority Queue".to_string(),
        directory_name: "heap_priority_queue".to_string(),
        easy_lesson: Some(vec![
            LeetcodeExercise::new(
                "kth-largest-element-in-a-stream",
                "Kth Largest Element in a Stream",
            ),
            LeetcodeExercise::new("last-stone-weight", "Last Stone Weight"),
        ]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new("k-closest-points-to-origin", "K Closest Points to Origin"),
            LeetcodeExercise::new(
                "kth-largest-element-in-an-array",
                "Kth Largest Element in an Array",
            ),
            LeetcodeExercise::new("task-scheduler", "Task Scheduler"),
            LeetcodeExercise::new("design-twitter", "Design Twitter"),
        ]),
        hard_lesson: Some(vec![LeetcodeExercise::new(
            "find-median-from-data-stream",
            "Find Median from Data Stream",
        )]),
    };
    course.course_builder()
}
