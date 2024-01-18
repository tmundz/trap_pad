pub struct Cursor {
    row: i32,
    col: i32,
    width: u32,
    height: u32,
    visible: bool,
}

impl Cursor {
    pub fn new(row: i32, col: i32, width: u32, height: u32) -> Self {
        Cursor {
            row,
            col,
            width,
            height,
            visible: true,
        }
    }

    pub fn move_left(&mut self) {
        self.col -= 1;
        //stop at cetian index
    }

    pub fn move_right(&mut self) {
        self.col += 1;
        //add logic to stop at where the line col max is
        //only update line col max if there is an ascii char typed
    }

    // add logic to remember the position it should be at when scrolling down
    pub fn move_up(&mut self) {
        self.row -= 1;
        // add logic to stop at the top
    }

    pub fn move_down(&mut self) {
        self.row += 1;
        // will need to add scroll
    }
}
