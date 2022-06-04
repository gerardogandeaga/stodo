use super::LineToken;

const GUTTER_STRING_INIT_CAP: usize = 256;

pub struct Gutter {
    string: String,
    width: usize,
}

impl Gutter {

    pub fn new() -> Self {
        Gutter { 
            string: String::with_capacity(GUTTER_STRING_INIT_CAP),
            width: 0,
        }
    }

    pub fn compile(&mut self, line_tokens: &Vec<LineToken>) -> String {
        self.set_gutter_width(line_tokens);

        line_tokens.iter()
            .for_each(|x| 
                self.append_to_gutter(x)
            );

        String::from(&self.string)
    }

    fn append_to_gutter(
        &mut self,
        line_token: &LineToken) {

        match line_token {
            LineToken::Stodo(line) => self.append(line.to_string().as_str()),
            LineToken::Div => self.append_div(),
            _ => {
                self.append("");
            }
        }
    }

    fn set_gutter_width(&mut self, line_tokens: &Vec<LineToken>) {
        // let n_digits = |x: u32| (u32::log10(x) + 1) as u8;
        let mut max_line_number: u32 = 0;

        for token in line_tokens.iter() {
            match token {
                LineToken::Stodo(line) => max_line_number = if *line > max_line_number { *line } else { max_line_number },
                _ => {},
            }
        }

        // find the number of digits in the number
        let mut d: u8 = 0;
        let mut n: u32 = max_line_number;
        
        if n == 0 {
            d = 1;
        }
        else {
            while n > 0 {
                n /= 10;
                d += 1;
            }
        }
        
        self.width = d as usize;
    }

    fn append_div(&mut self) {
        self.string.push_str( 
            format!("{: >width$}", "", width = self.width)
                .as_str()
        );
        self.string.push_str("├ ") ; // │
        self.string.push('\n');
    }

    fn append(&mut self, s: &str) {
        self.string.push_str( 
            format!("{: >width$}", s, width = self.width)
                .as_str()
        );
        self.string.push_str("│ ") ; // │
        self.string.push('\n');
    }

}
