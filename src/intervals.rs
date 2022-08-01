use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::intervals");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Intervals".to_string(),
        directory_name: "intervals".to_string(),
        easy_lesson: Some(vec![LeetcodeExercise::new(
            "meeting-rooms",
            "Meeting Rooms",
        )]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new("meeting-rooms-ii", "Meeting Rooms II"),
            LeetcodeExercise::new("insert-interval", "Insert Interval"),
            LeetcodeExercise::new("merge-intervals", "Merge Intervals"),
            LeetcodeExercise::new("non-overlapping-intervals", "Non-overlapping Intervals"),
        ]),
        hard_lesson: Some(vec![LeetcodeExercise::new(
            "minimum-interval-to-include-each-query",
            "Minimum Interval to Include Each Query",
        )]),
    };
    course.course_builder()
}
