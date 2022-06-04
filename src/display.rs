#[allow(dead_code)]

mod gutter;
mod forest;
pub mod displayable;
pub mod builder;

#[derive(Debug, Clone, Copy)]
pub enum LineToken {
    RootDir,
    Dir,
    File,
    Stodo(u32),
    Div,
    Empty,
}

#[derive(Debug, Clone)]
pub struct StodoOutput {
    line_tokens: Vec<LineToken>,
    out_strings: Vec<String>,
}

impl StodoOutput {

    pub fn print_tokens(&self) {
        self.line_tokens.iter().enumerate().for_each(|(i, t)| {
            println!("{} - {:?}", i, t);
        });
    }

    pub fn print_strings(&self) {
        self.out_strings.iter().enumerate().for_each(|(i, s)| {
            println!("{} - {}", i, s);
        });
    }

    pub fn len(&self) -> usize {
        self.out_strings.len()
    }

    pub fn window(&self, start: usize, end: usize) -> Vec<(&LineToken, &String)> {
        let tokens = self.line_tokens[start..end].into_iter();
        let strings = self.out_strings[start..end].into_iter();
        tokens.zip(strings).collect()
    }

    pub fn at(&self, i: usize) -> (&LineToken, &String) {
        (&self.line_tokens[i], &self.out_strings[i])
    }

    pub fn is_stodo_at(&self, i: usize) -> bool {
        match self.line_tokens[i] {
            LineToken::Stodo(_) => true,
            _ => false,
        }
    }

    pub fn is_rootdir_at(&self, i: usize) -> bool {
        match self.line_tokens[i] {
            LineToken::RootDir => true,
            _ => false,
        }
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn multi_struct_tree() -> Result<(), String> {
//         Ok(())
//     }
// }
