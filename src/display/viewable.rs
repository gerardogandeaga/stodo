use crate::core::StodoForest;

pub trait Viewable {
    fn compile(&mut self, stodo_forest: &StodoForest) -> Option<String>;
}
