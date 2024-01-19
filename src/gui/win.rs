extern crate sdl2;

use std::borrow::Borrow;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::editor::Editor;

// if I want to make it more usable it will need to adapt and be more flexible
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub fn win_init(sdl_context: &sdl2::Sdl) -> Result<(Canvas<Window>, EventPump), String> {
    let video_subsystem = sdl_context.video()?;

    //create main window
    let window = video_subsystem
        .window("trap_pad", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let event_pump = sdl_context.event_pump()?;
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    Ok((canvas, event_pump))
}

pub fn run_loop(
    mut canvas: Canvas<Window>,
    mut event_pump: EventPump,
    editor: &mut Editor,
) -> Result<(), String> {
    // Main loop
    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::KeyDown {
                keycode: Some(keycode),
                ..
            } = event
            {
                match keycode {
                    Keycode::Left => editor.move_left(),
                    Keycode::Right => editor.move_right(),
                    Keycode::Up => editor.move_up(),
                    Keycode::Down => editor.move_down(),
                    Keycode::Escape => break 'running,
                    _ => {}
                }
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        draw_line(&mut canvas, editor)?;
        draw_cursor(&mut canvas, editor)?;
        canvas.present();
        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}

fn draw_cursor(canvas: &mut Canvas<Window>, editor: &mut Editor) -> Result<(), String> {
    editor.get_cursor_mut().update();

    if editor.get_cursor_mut().get_visible() {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let cursor_rect = sdl2::rect::Rect::new(
            editor.get_cursor_mut().get_col(),
            editor.get_cursor_mut().get_row(),
            editor.get_cursor_mut().get_width(),
            editor.get_cursor_mut().get_height(),
        );
        canvas.fill_rect(cursor_rect)?;
    }

    Ok(())
}

fn draw_line(canvas: &mut Canvas<Window>, editor: &mut Editor) -> Result<(), String> {
    for i in 0..editor.get_lines_mut().len() {
        let line = editor.get_line(i).clone();
        // println!("{}", line.cursor_is_present());
        if editor.get_line(i).cursor_is_present() {
            canvas.set_draw_color(Color::RGBA(49, 25, 109, 128));
            let line_rect = sdl2::rect::Rect::new(
                0,
                line.get_cur_min() as i32,
                line.get_width(),
                line.get_height(),
            );
            canvas.fill_rect(line_rect)?;
        }
    }
    Ok(())
}
