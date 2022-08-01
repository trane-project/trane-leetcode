use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::graphs");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Graphs".to_string(),
        directory_name: "graphs".to_string(),
        easy_lesson: None,
        medium_lesson: Some(vec![
            LeetcodeExercise::new("number-of-islands", "Number of Islands"),
            LeetcodeExercise::new("clone-graph", "Clone Graph"),
            LeetcodeExercise::new("max-area-of-island", "Max Area of Island"),
            LeetcodeExercise::new("pacific-atlantic-water-flow", "Pacific Atlantic Water Flow"),
            LeetcodeExercise::new("surrounded-regions", "Surrounded Regions"),
            LeetcodeExercise::new("rotting-oranges", "Rotting Oranges"),
            LeetcodeExercise::new("walls-and-gates", "Walls and Gates"),
            LeetcodeExercise::new("course-schedule", "Course Schedule"),
            LeetcodeExercise::new("course-schedule-ii", "Course Schedule II"),
            LeetcodeExercise::new("redundant-connection", "Redundant Connection"),
            LeetcodeExercise::new(
                "number-of-connected-components-in-an-undirected-graph",
                "Number of Connected Components in an Undirected Graph",
            ),
        ]),
        hard_lesson: Some(vec![LeetcodeExercise::new("word-ladder", "Word Ladder")]),
    };
    course.course_builder()
}
