//! Code to generate Leetcode courses.
mod advanced_graphs;
mod arrays_hashing;
mod backtracking;
mod binary_search;
mod bit_manipulation;
mod graphs;
mod greedy;
mod heap_priority_queue;
mod intervals;
mod leetcode;
mod linked_list;
mod math_and_geometry;
mod one_d_dynamic_programming;
mod sliding_window;
mod stack;
mod trees;
mod tries;
mod two_d_dynamic_programming;
mod two_pointers;

use std::path::Path;

use anyhow::Result;

fn build_courses(library_root: &Path) -> Result<()> {
    let course_builders = vec![
        advanced_graphs::course_builder(),
        arrays_hashing::course_builder(),
        backtracking::course_builder(),
        binary_search::course_builder(),
        bit_manipulation::course_builder(),
        graphs::course_builder(),
        greedy::course_builder(),
        heap_priority_queue::course_builder(),
        intervals::course_builder(),
        linked_list::course_builder(),
        math_and_geometry::course_builder(),
        one_d_dynamic_programming::course_builder(),
        sliding_window::course_builder(),
        stack::course_builder(),
        trees::course_builder(),
        tries::course_builder(),
        two_d_dynamic_programming::course_builder(),
        two_pointers::course_builder(),
    ];

    for course_builder in course_builders {
        course_builder.build(library_root)?;
        println!("Built {} course", course_builder.course_manifest.name);
    }
    Ok(())
}

fn main() -> Result<()> {
    let curr_dir = std::env::current_dir()?;
    build_courses(curr_dir.as_path())
}

#[cfg(test)]
mod tests {
    use trane::scheduler::ExerciseScheduler;

    use crate::build_courses;

    #[test]
    fn open_library() -> anyhow::Result<()> {
        let temp_dir = tempfile::TempDir::new()?;
        let library_root = &temp_dir.path().to_path_buf();
        build_courses(library_root)?;
        let trane = trane::Trane::new(library_root.to_str().unwrap())?;
        let batch = trane.get_exercise_batch(None)?;
        assert!(!batch.is_empty());
        Ok(())
    }
}
