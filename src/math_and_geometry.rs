use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::math_and_geometry");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Math & Geometry".to_string(),
        directory_name: "math_and_geometry".to_string(),
        easy_lesson: Some(vec![
            LeetcodeExercise::new("happy-number", "Happy Number"),
            LeetcodeExercise::new("plus-one", "Plus One"),
        ]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new("rotate-image", "Rotate Image"),
            LeetcodeExercise::new("spiral-matrix", "Spiral Matrix"),
            LeetcodeExercise::new("set-matrix-zeroes", "Set Matrix Zeroes"),
            LeetcodeExercise::new("powx-n", "Pow(x, n)"),
            LeetcodeExercise::new("multiply-strings", "Multiply Strings"),
            LeetcodeExercise::new("detect-squares", "Detect Squares"),
        ]),
        hard_lesson: None,
    };
    course.course_builder()
}
