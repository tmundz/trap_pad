/***********************************************************************
 * Purpose: Holds the logic for the cursor
 * This means it needs to track the position of the row and column
 *
 * *********************************************************************/

use std::time;

use crate::gui::win::{HEIGHT, WIDTH};

pub const STEP: i32 = 20;
const BLINK: time::Duration = time::Duration::from_millis(500);

#[derive(Copy, Clone)]
pub struct Cursor {
    row: i32,
    col: i32,
    width: u32,
    height: u32,
    visible: bool,
    recent_blink: time::Instant,
    cur_col_index: usize, // will need to update on left right movement
    cur_row_index: usize,
}

impl Cursor {
    pub fn new(row: i32, col: i32, width: u32, height: u32) -> Self {
        Cursor {
            row,
            col,
            width,
            height,
            visible: true,
            recent_blink: time::Instant::now(),
            cur_col_index: 0,
            cur_row_index: 0,
        }
    }

    /*
     * CURSOR ATTRIBUTES
     */
    // Updates the visibility after a certain time
    //
    // issue and fix mut only was updating the current copy not the actual struct
    pub fn update(&mut self) {
        if self.recent_blink.elapsed() >= BLINK {
            self.visible = !self.visible;
            self.recent_blink = time::Instant::now();
        }
    }

    /*
     * MOVING FUNCTIONS
     */
    pub fn move_left(&mut self) {
        if !self.col_edge(0) {
            self.col -= STEP;
        }
        //stop at cetian index
    }

    pub fn move_right(&mut self) {
        self.col += STEP;
        //add logic to stop at where the line col max is
        //only update line col max if there is an ascii char typed
    }

    // add logic to remember the position it should be at when scrolling down
    pub fn move_up(&mut self) {
        self.row -= STEP;
        self.cur_row_index -= 1;
    }

    pub fn move_down(&mut self) {
        self.row += STEP;
        self.cur_row_index += 1;
    }

    /*
     * GET FUNCTIONS
     */
    pub fn get_width(self) -> u32 {
        self.width
    }

    pub fn get_height(self) -> u32 {
        self.height
    }

    pub fn get_row(self) -> i32 {
        self.row
    }

    pub fn get_col(self) -> i32 {
        self.col
    }

    pub fn get_visible(self) -> bool {
        self.visible
    }

    pub fn get_cur_row_index(self) -> usize {
        self.cur_row_index
    }

    //TODO MAY NEED TO CHANGE IF I ADD SCROLLING
    pub fn row_edge(self, flag: i32) -> bool {
        //0 if left
        //1 if right
        (flag == 0 && (self.row - STEP) <= 0) || (flag == 1 && (self.row + STEP) >= HEIGHT as i32)
    }

    pub fn col_edge(self, flag: i32) -> bool {
        //0 if up
        //1 if down
        (flag == 0 && (self.col - STEP) <= 0) || (flag == 1 && (self.col + STEP) >= WIDTH as i32)
    }
}
