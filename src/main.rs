mod editor;
mod gui;
use editor::cursor::Cursor;
use gui::win;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let (canvas, event_pump) = win::win_init(&sdl_context)?;

    let mut cursor = Cursor::new(10, 10, 2, 20);
    let _ = win::run_loop(canvas, event_pump, &mut cursor);
    Ok(())
}
