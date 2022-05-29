// TODO: refactor this file into display.rs
use super::gutter::Gutter;
use super::forest::Forest;

use crate::core::{StodoForest};

pub enum LineToken {
    RootDir,
    Dir,
    File,
    Stodo(u32),
    Empty,
}

pub struct DisplayForestBuilder {
    gutter: Gutter,
    forest: Forest,
}

impl DisplayForestBuilder {

    fn new() -> Self {
        Self {
            gutter: Gutter::new(),
            forest: Forest::new(),
        }
    }

    pub fn compile(stodo_forest: &StodoForest) -> String {
        let mut display_forest_builder: DisplayForestBuilder = Self::new();

        display_forest_builder.forest.compile(stodo_forest);

        let gutter_str: String = display_forest_builder.gutter.compile(display_forest_builder.forest.line_tokens());
        let forest_str: String = display_forest_builder.forest.to_string();

        let mut display_str: String = String::with_capacity(gutter_str.len() + forest_str.len());

        gutter_str.split("\n").zip(forest_str.split("\n"))
            .for_each(|(g, f)| {
                display_str.push_str(g);
                display_str.push_str(f);
                display_str.push('\n');
            });

        display_str.trim_end().to_string()
    }
}

