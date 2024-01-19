/***********************************************************************
 * Purpose: Holds the logic and attributes for each line
 * I want to have a box showing the what the line is
 *
 * *********************************************************************/

use crate::win::{HEIGHT, WIDTH};

#[derive(Clone)]
pub struct Line {
    abs_min: u32,
    abs_max: u32,
    cur_max: u32, // row max/min will adjust as items get added
    cur_min: u32,
    //for drawing
    height: u32,
    width: u32,

    // line height should be about 20px
    // cursor column postion
    contents: String,
    cursor_present: bool,
}

// needs to move with cursor will update on each down or up
impl Line {
    pub fn new(cur_min: u32, cur_max: u32, cursor_present: bool) -> Self {
        Line {
            abs_min: 0,
            abs_max: HEIGHT,
            height: 20,
            width: WIDTH,
            cur_max,
            cur_min,
            contents: String::new(),
            cursor_present,
        }
    }

    pub fn set_present(&mut self, present: bool) {
        self.cursor_present = present;
    }

    pub fn cursor_is_present(&self) -> bool {
        self.cursor_present
    }

    /*
     * GET FUNCTIONS
     */
    pub fn get_cur_min(&self) -> u32 {
        self.cur_min
    }

    pub fn get_cur_max(&self) -> u32 {
        self.cur_max
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }
}
