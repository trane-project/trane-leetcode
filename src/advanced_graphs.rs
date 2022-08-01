use crate::{
    graphs,
    leetcode::{LeetcodeCourse, LeetcodeExercise},
};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::advanced_graphs");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![*graphs::COURSE_ID],
        topic: "Advanced Graphs".to_string(),
        directory_name: "advanced_graphs".to_string(),
        easy_lesson: None,
        medium_lesson: Some(vec![
            LeetcodeExercise::new(
                "min-cost-to-connect-all-points",
                "Min Cost to Connect All Points",
            ),
            LeetcodeExercise::new("network-delay-time", "Network Delay Time"),
            LeetcodeExercise::new(
                "cheapest-flight-within-k-stops",
                "Cheapest Flight Within K Stops",
            ),
        ]),
        hard_lesson: Some(vec![
            LeetcodeExercise::new("reconstruct-itinerary", "Reconstruct Itinerary"),
            LeetcodeExercise::new("swim-in-rising-water", "Swim in Rising Water"),
            LeetcodeExercise::new("alien-dictionary", "Alien Dictionary"),
        ]),
    };
    course.course_builder()
}
