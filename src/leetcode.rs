use std::{collections::BTreeMap, vec};

use indoc::{formatdoc, indoc};
use trane::{
    course_builder::{AssetBuilder, CourseBuilder, ExerciseBuilder, LessonBuilder},
    data::{
        BasicAsset, CourseManifest, ExerciseAsset, ExerciseManifestBuilder, ExerciseType,
        LessonManifestBuilder,
    },
};
use ustr::Ustr;

static AUTHORS: &str = "The Trane Project";

// A struct representing a single Leetcode exercise.
pub struct LeetcodeExercise {
    // The partial ID of the exercise. The actual ID is constructed by prepending the lesson ID.
    // This ID should be the same value used in the Leetcode URL.
    pub partial_id: String,

    // The human readable name of the exercise.
    pub name: String,
}

impl LeetcodeExercise {
    pub fn new(partial_id: &str, name: &str) -> Self {
        Self {
            partial_id: partial_id.to_string(),
            name: name.to_string(),
        }
    }

    pub fn exercise_builder(&self, lesson_id: &str) -> ExerciseBuilder {
        let exercise_id = format!("{}::{}", lesson_id, self.partial_id);
        let name_clone = self.name.clone();
        let url = format!("https://leetcode.com/problems/{}", self.partial_id);

        ExerciseBuilder {
            directory_name: self.partial_id.clone(),
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(exercise_id.clone())
                    .name(name_clone.clone())
                    .clone()
            }),
            asset_builders: vec![
                AssetBuilder {
                    file_name: "front.md".to_string(),
                    contents: formatdoc! {"
                        Solve the Leetcode problem {}.
                        The URL of the problem is [{}]({}).
                    ", self.name, url, url},
                },
                AssetBuilder {
                    file_name: "back.md".to_string(),
                    contents: indoc! {"
                        You can find a solution to the problem on [neetcode.io](neetcode.io).
                    "}
                    .to_string(),
                },
            ],
        }
    }
}

// A struct used to build a Leetcode course.
pub struct LeetcodeCourse {
    // The ID of the course.
    pub id: Ustr,

    // The dependencies of this course.
    pub dependencies: Vec<Ustr>,

    // The topic covered by this course.
    pub topic: String,

    // The name of the directory under to which the course is stored.
    pub directory_name: String,

    // The easy problems in this course.
    pub easy_lesson: Option<Vec<LeetcodeExercise>>,

    // The medium problems in this course.
    pub medium_lesson: Option<Vec<LeetcodeExercise>>,

    // The hard problems in this course.
    pub hard_lesson: Option<Vec<LeetcodeExercise>>,
}

impl LeetcodeCourse {
    pub fn lesson_builder(
        &self,
        difficulty: &str,
        exercises: &[LeetcodeExercise],
        dependencies: Vec<Ustr>,
    ) -> LessonBuilder {
        let lesson_id = format!("{}::{}", self.id, difficulty);
        let lesson_id_clone = lesson_id.clone();
        let lesson_name = format!("{} - {}", self.topic, difficulty);
        LessonBuilder {
            directory_name: difficulty.to_string(),
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(lesson_id_clone.clone())
                    .name(lesson_name.clone())
                    .dependencies(dependencies.clone())
                    .clone()
            }),
            exercise_manifest_template: ExerciseManifestBuilder::default()
                .course_id(self.id)
                .lesson_id(lesson_id.clone())
                .exercise_type(ExerciseType::Procedural)
                .exercise_asset(ExerciseAsset::FlashcardAsset {
                    front_path: "front.md".to_string(),
                    back_path: Some("back.md".to_string()),
                })
                .clone(),
            exercise_builders: exercises
                .iter()
                .map(|exercise| exercise.exercise_builder(&lesson_id))
                .collect(),
            asset_builders: vec![],
        }
    }

    pub fn course_builder(&self) -> CourseBuilder {
        let easy_id = Ustr::from(format!("{}::easy", self.id).as_str());
        let medium_id = Ustr::from(format!("{}::medium", self.id).as_str());

        let mut lesson_builders: Vec<LessonBuilder> = Vec::default();
        if let Some(easy_lesson) = &self.easy_lesson {
            lesson_builders.push(self.lesson_builder("easy", easy_lesson, vec![]));
        }

        if let Some(medium_lesson) = &self.medium_lesson {
            let dependencies = if self.easy_lesson.is_some() {
                vec![easy_id]
            } else {
                vec![]
            };

            lesson_builders.push(self.lesson_builder("medium", medium_lesson, dependencies));
        }

        if let Some(hard_lesson) = &self.hard_lesson {
            let dependencies = if self.medium_lesson.is_some() {
                vec![medium_id]
            } else if self.easy_lesson.is_some() {
                vec![easy_id]
            } else {
                vec![]
            };

            lesson_builders.push(self.lesson_builder("hard", hard_lesson, dependencies));
        }

        CourseBuilder {
            directory_name: self.directory_name.clone(),
            course_manifest: CourseManifest {
                id: self.id,
                name: self.topic.clone(),
                authors: Some(vec![AUTHORS.to_string()]),
                description: Some(format!(
                    "Solve Leetcode problems related to the topic {}",
                    self.topic
                )),
                dependencies: self.dependencies.clone(),
                course_instructions: Some(BasicAsset::MarkdownAsset {
                    path: "instructions.md".to_string(),
                }),
                course_material: None,
                metadata: Some(BTreeMap::from([(
                    "leetcode_topic".to_string(),
                    vec![self.topic.clone()],
                )])),
                generator_config: None,
            },
            lesson_manifest_template: LessonManifestBuilder::default().course_id(self.id).clone(),
            lesson_builders,
            asset_builders: vec![AssetBuilder {
                file_name: "instructions.md".to_string(),
                contents: indoc! {"
                    Make progress in this course by solving the given Leetcode problems.
                    
                    What solving a problem means is open-ended. You could code the solution from
                    scratch every time, or do it only the first time and just review the solution
                    any subsequent time you are given the problem.

                    You can find the video solutions to all the problems on
                    [neetcode.io](neetcode.io).
                "}
                .to_string(),
            }],
        }
    }
}
