mod cursor;
mod line;

use crate::gui::win::{HEIGHT, WIDTH};
use cursor::Cursor;
use line::Line;

#[derive(Clone)]
pub struct Editor {
    lines: Vec<Line>,
    cursor: Cursor,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            cursor: Cursor::new(20, 20, 2, 20),
            lines: init_lines(),
        }
    }

    /*
     * MOVING FUNCTIONS
     *
     * TODO fix bug where if I press to many keys in an edge cause overflow
     */
    pub fn move_left(&mut self) {
        if !self.cursor.col_edge(0) {
            self.cursor.move_left();
        }
        //stop at cetian index
    }

    pub fn move_right(&mut self) {
        if !self.cursor.col_edge(1) {
            self.cursor.move_right();
        }
        //add logic to stop at where the line col max is
        //only update line col max if there is an ascii char typed
    }

    // add logic to remember the position it should be at when scrolling down
    pub fn move_up(&mut self) {
        if !self.cursor.row_edge(0) {
            self.lines[self.cursor.get_cur_row_index()].set_present(false);
            self.cursor.move_up();
            self.lines[self.cursor.get_cur_row_index()].set_present(true);
        }
        // add logic to stop at the top
    }

    pub fn move_down(&mut self) {
        if !self.cursor.row_edge(1) {
            self.lines[self.cursor.get_cur_row_index()].set_present(false);
            self.cursor.move_down();
            self.lines[self.cursor.get_cur_row_index()].set_present(true);
        }
        // will need to add scroll
    }

    /*
     * GET MUT FUNCTIONS
     */
    pub fn get_cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn get_line_mut(&mut self, pos: usize) -> &mut Line {
        &mut self.lines[pos]
    }

    pub fn get_lines_mut(&mut self) -> &mut Vec<Line> {
        &mut self.lines
    }

    /*
     * GET FUNCTIONS
     */
    pub fn get_cursor(self) -> Cursor {
        self.cursor
    }

    pub fn get_line(&self, pos: usize) -> Line {
        self.lines[pos].clone()
    }

    pub fn get_lines(self) -> Vec<Line> {
        self.lines
    }
}

fn init_lines() -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    for i in (20..HEIGHT).step_by(cursor::STEP as usize) {
        lines.push(Line::new(i, WIDTH, false));
        println!("{}", i);
    }
    lines[0].set_present(true);

    lines
}
