use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
enum GutterEntry {
    LineNumber(u32),
    Status(bool),
    Empty,
}

pub struct Gutter {
    content: Vec<(u32, Vec<GutterEntry>)>
}

impl Gutter {

    pub fn new() -> Self {
        Gutter { 
            content: vec![
                (1, vec![GutterEntry::Empty])
            ]
        }
    }
}

impl fmt::Display for Gutter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {:?}", self.content[0].0, self.content[0].1)
    }
}
