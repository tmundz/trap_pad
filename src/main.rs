mod editor;
mod gui;
use editor::Editor;
use gui::win;
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let (canvas, event_pump) = win::win_init(&sdl_context)?;
    let mut editor = Editor::new();
    let _ = win::run_loop(canvas, event_pump, &mut editor);
    Ok(())
}
