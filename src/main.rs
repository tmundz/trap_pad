mod editor;
mod gui;
use editor::cursor::Cursor;
use editor::line::Line;
use gui::win;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let (canvas, event_pump) = win::win_init(&sdl_context)?;

    let mut cursor = Cursor::new(20, 20, 2, 20);
    let line = Line::new(0, 800, true);
    let _ = win::run_loop(canvas, event_pump, &mut cursor, line);
    Ok(())
}
