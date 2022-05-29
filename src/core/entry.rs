use ansi_term::Colour;

#[derive(Debug)]

pub enum StodoEntry {
    TODO(String, u32),
    FIXME(String, u32)
}

// pub struct StodoEntry(String, u32);

impl StodoEntry {
    
    pub fn todo(str: String, line_number: u32) -> Self {
        Self::TODO(str, line_number)
    }

    pub fn fixme(str: String, line_number: u32) -> Self {
        Self::FIXME(str, line_number)
    }
}

impl StodoEntry {

    pub fn stodo_string(&self) -> String {
        match self {
            Self::TODO(str, _) => String::from(str),
            Self::FIXME(str, _) => String::from(str)
        }
    }

    pub fn line_number(&self) -> u32 {
        match self {
            Self::TODO(_, line_number) => *line_number,
            Self::FIXME(_, line_number) => *line_number,
        }
    }
}
