use super::{StodoOutput, LineToken};

#[allow(dead_code)]
#[derive(Debug)]
pub struct TermData {
    buffer: StodoOutput,
    cursor: usize,
    render_top: usize,
    window_dims: (u16, u16), // (height, width)
}

impl TermData {

    pub fn new(stodo_output: StodoOutput, dims: (u16, u16)) -> Self {
        Self {
            buffer: stodo_output,
            cursor: 0,
            render_top: 0,
            window_dims: dims
        }
    }

    /// resizes the buffer window and tranforms the render top so that the cursor is always kept on the sreen
    pub fn resize_window(&mut self, dims: (u16, u16)) {
        self.window_dims = dims;
        let render_bottom: usize = self.render_bottom();

        if self.cursor < self.render_top {
            self.shift_top_up(self.render_top - self.cursor);
        }
        else 
        if self.cursor > render_bottom {
            self.shift_top_down(self.cursor - render_bottom);
        }
    }

    /// Returns the content segment needed for immediate rendering
    pub fn buffer_window(&self) -> Vec<(&LineToken, &String)> {
        let window_height: usize = self.window_dims.0 as usize;
        let bottom: usize = usize::min(self.render_top + window_height, self.buffer.len());
        self.buffer.window(self.render_top, bottom)
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }

    /// moves the buffer pointer to the next stodo. If there is none then the pointer doesnt change.
    /// Returns the number of lines the pointer moved by.
    pub fn point_to_next(&mut self) -> usize {
        let end: usize = self.buffer.len() - 1;
        if self.cursor >= end {
            return 0;
        }

        let mut i: usize = self.cursor + 1;
        while i < end && !(self.buffer.is_stodo_at(i) || self.buffer.is_rootdir_at(i)) {
            i += 1
        }

        let diff: usize = i - self.cursor;
        self.cursor = i;

        // shift the render top if needed
        let render_bottom: usize = self.render_bottom();
        if self.cursor > render_bottom { 
            self.shift_top_down(self.cursor - render_bottom);
        }

        diff
    }

    /// moves the buffer pointer to the prev stodo. If there is none then the pointer doesnt change.
    /// Returns the number of lines the pointer moved by.
    pub fn point_to_prev(&mut self) -> usize {
        if self.cursor == 0 {
            return 0;
        }

        let mut i: usize = self.cursor - 1;
        while i > 0 && !(self.buffer.is_stodo_at(i) || self.buffer.is_rootdir_at(i)) {
            i -= 1
        }

        let diff: usize = self.cursor - i;
        self.cursor = i;

        // shift the render top if needed
        if self.cursor < self.render_top { 
            self.shift_top_up(self.render_top - self.cursor);
        }

        diff
    }

    /// Changes the locaton of the top window
    fn shift_top_down(&mut self, amount: usize) {
        if self.render_top < self.buffer.len() - 1 {
            self.render_top += amount;
        }
    }

    fn shift_top_up(&mut self, amount: usize) {
        if self.render_top > 0 {
            self.render_top -= amount;
        }
    }

    /// Transforms cursor positon on the data to a cursor positon within the window
    pub fn cursor_window_position(&self) -> u16 {
        (self.cursor - self.render_top) as u16
    }

    fn render_bottom(&self) -> usize {
        self.render_top + self.window_dims.0 as usize - 1
    }
}
