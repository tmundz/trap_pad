/***********************************************************************
 * Purpose: Holds the logic and attributes for each line
 * I want to have a box showing the what the line is
 *
 * *********************************************************************/

#[derive(Clone)]
pub struct Line {
    abs_min: i32,
    abs_max: i32,
    cur_max: i32, // row max/min will adjust as items get added
    cur_min: i32,
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
    pub fn new(abs_min: i32, abs_max: i32, cursor_present: bool) -> Self {
        Line {
            abs_min,
            abs_max,
            height: 20,
            width: abs_max as u32,
            cur_max: 0,
            cur_min: 0,
            contents: String::new(),
            cursor_present,
        }
    }

    /*
     * GET FUNCTIONS
     */
    pub fn get_abs_min(self) -> i32 {
        self.abs_min
    }

    pub fn get_abs_max(self) -> i32 {
        self.abs_max
    }

    pub fn get_height(self) -> u32 {
        self.height
    }

    pub fn get_width(self) -> u32 {
        self.width
    }
}
