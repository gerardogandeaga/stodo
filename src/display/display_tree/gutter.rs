use super::builder::LineToken;

const GUTTER_STRING_INIT_CAP: usize = 256;

pub struct Gutter;

impl Gutter {

    pub fn new() -> Self {
        Gutter { }
    }

    pub fn compile(&self, line_tokens: &Vec<LineToken>) -> String {
        let mut gutter_string = String::with_capacity(GUTTER_STRING_INIT_CAP);
        let width = self.gutter_width(line_tokens) as usize;

        line_tokens.iter()
            .for_each(|x| 
                Gutter::append_to_gutter_string(
                    &mut gutter_string, 
                    width, 
                    x
                )
            );

        gutter_string
    }

    fn append_to_gutter_string(
        gutter_string: &mut String, 
        gutter_width: usize,
        line_token: &LineToken) {

        let mut gutter_append = |s: &str| {
            gutter_string.push_str( 
                format!("{: >width$}", s, width = gutter_width)
                    .as_str()
            );
            gutter_string.push_str("| ") ; // │
            gutter_string.push('\n');
        };

        match line_token {
            LineToken::Stodo(line) => {
                gutter_append(line.to_string().as_str())
            }
            _ => {
                gutter_append("");
            }
        }
    }

    fn gutter_width(&self, line_tokens: &Vec<LineToken>) -> u8 {
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
        
        d
    }

}
