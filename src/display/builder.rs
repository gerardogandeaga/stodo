// TODO: refactor this file into display.rs
use super::gutter::Gutter;
use super::forest::Forest;

use crate::core::{StodoForest};
use super::{LineToken, StodoOutput};

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


    pub fn compile(stodo_forest: &StodoForest) -> StodoOutput {
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

        let tokens: Vec<LineToken> = display_forest_builder.forest.line_tokens().iter().map(|lt| lt.clone()).collect();
        let lines: Vec<String> = display_str.trim_end().split("\n").map(|l| String::from(l)).collect();

        assert!(tokens.len() == lines.len(), "Number of tokens must equal the number of lines!");
        let output: StodoOutput = StodoOutput { 
            line_tokens: tokens,
            out_strings: lines
        };

        // display_str.trim_end().to_string()
        output
    }
}
