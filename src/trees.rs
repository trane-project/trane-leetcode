use crate::leetcode::{LeetcodeCourse, LeetcodeExercise};
use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::leetcode::trees");
}

pub fn course_builder() -> CourseBuilder {
    let course = LeetcodeCourse {
        id: *COURSE_ID,
        dependencies: vec![],
        topic: "Trees".to_string(),
        directory_name: "trees".to_string(),
        easy_lesson: Some(vec![
            LeetcodeExercise::new("invert-binary-tree", "Invert Binary Tree"),
            LeetcodeExercise::new(
                "maximum-depth-of-binary-tree",
                "Maximum Depth of Binary Tree",
            ),
            LeetcodeExercise::new("diameter-of-binary-tree", "Diameter of Binary Tree"),
            LeetcodeExercise::new("balanced-binary-tree", "Balanced Binary Tree"),
            LeetcodeExercise::new("same-tree", "Same Tree"),
            LeetcodeExercise::new("subtree-of-another-tree", "Subtree of Another Tree"),
            LeetcodeExercise::new(
                "lowest-common-ancestor-of-a-binary-search-tree",
                "Lowest Common Ancestor of a Binary Search Tree",
            ),
        ]),
        medium_lesson: Some(vec![
            LeetcodeExercise::new(
                "binary-tree-level-order-traversal",
                "Binary Tree Level Order Traversal",
            ),
            LeetcodeExercise::new("binary-tree-right-side-view", "Binary Tree Right Side View"),
            LeetcodeExercise::new(
                "count-good-nodes-in-binary-tree",
                "Count Good Nodes in Binary Tree",
            ),
            LeetcodeExercise::new("validate-binary-search-tree", "Validate Binary Search Tree"),
            LeetcodeExercise::new(
                "kth-smallest-element-in-a-bst",
                "Kth Smallest Element in a BST",
            ),
            LeetcodeExercise::new(
                "construct-tree-from-preorder-and-inorder-traversal",
                "Construct Tree from Preorder and Inorder Traversal",
            ),
        ]),
        hard_lesson: Some(vec![
            LeetcodeExercise::new("binary-tree-max-path-sum", "Binary Tree Max Path Sum"),
            LeetcodeExercise::new(
                "serialize-and-deserialize-binary-tree",
                "Serialize and Deserialize Binary Tree",
            ),
        ]),
    };
    course.course_builder()
}
