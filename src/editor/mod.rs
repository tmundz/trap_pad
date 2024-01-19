mod cursor;
mod line;

use cursor::Cursor;
use line::Line;

#[derive(Clone)]
pub struct Editor {
    lines: Line,
    cursor: Cursor,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            cursor: Cursor::new(20, 20, 2, 20),
            lines: Line::new(0, 800, true),
        }
    }

    /*
     * GET MUT FUNCTIONS
     */
    pub fn get_cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn get_line_mut(&mut self) -> &mut Line {
        &mut self.lines
    }

    /*
     * GET FUNCTIONS
     */
    pub fn get_cursor(self) -> Cursor {
        self.cursor
    }

    pub fn get_line(self) -> Line {
        self.lines
    }
}
