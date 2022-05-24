
#[allow(dead_code)]
#[derive(Debug)]
pub struct TermData {
    buffer: Vec<String>,
    pointer: usize,
    render_top: usize,
}

impl TermData {

    pub fn new(s: String) -> Self {
        Self {
            buffer: s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>(),
            pointer: 0,
            render_top: 0,
        }
    }

    pub fn buffer_window(&self, window_size: usize) -> std::slice::Iter<'_, String> {
        let bottom = usize::min(self.render_top + window_size, self.buffer.len());
        self.buffer[self.render_top..bottom].into_iter()
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }

    pub fn point_to_prev(&mut self) -> bool {
        if self.pointer < self.buffer.len() - 1 {
            self.pointer += 1;
            true
        }
        else {
            false
        }
    }

    pub fn point_to_next(&mut self) -> bool {
        if self.pointer > 0 {
            self.pointer -= 1;
            true
        }
        else {
            false
        }
    }

    pub fn shift_top_down(&mut self) {
        if self.render_top < self.buffer.len() - 1 {
            self.render_top += 1;
        }
    }

    pub fn shift_top_up(&mut self) {
        if self.render_top > 0 {
            self.render_top -= 1;
        }
    }
}
