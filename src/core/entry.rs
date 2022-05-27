use ansi_term::Colour;

#[derive(Debug)]
pub struct StodoEntry(String, u32);


impl StodoEntry {

    pub fn new(str: String, line_number: u32) -> Self {
        StodoEntry(str, line_number)
    }
}

impl StodoEntry {

    pub fn stodo_string(&self) -> String {
        String::from(&self.0)
    }

    pub fn line_number(&self) -> u32 {
        self.1
    }
}
