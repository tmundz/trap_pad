mod editor;
mod gui;
use editor::Cursor;
use gui::win;
fn main() -> Result<(), String> {
    let _ = win::win_init();
    Ok(())
}
