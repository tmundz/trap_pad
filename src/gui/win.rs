extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::editor::cursor::Cursor;

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

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let event_pump = sdl_context.event_pump()?;

    Ok((canvas, event_pump))
}

pub fn run_loop(
    mut canvas: Canvas<Window>,
    mut event_pump: EventPump,
    cursor: &mut Cursor,
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
                    Keycode::Left => cursor.move_left(),
                    Keycode::Right => cursor.move_right(),
                    Keycode::Up => cursor.move_up(),
                    Keycode::Down => cursor.move_down(),
                    Keycode::Escape => break 'running,
                    _ => {}
                }
            }
        }

        draw(&mut canvas, cursor)?;
        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}

fn draw(canvas: &mut Canvas<Window>, cursor: &mut Cursor) -> Result<(), String> {
    cursor.update();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    if cursor.get_visible() {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let cursor_rect = sdl2::rect::Rect::new(
            cursor.get_col(),
            cursor.get_row(),
            cursor.get_width(),
            cursor.get_height(),
        );
        canvas.fill_rect(cursor_rect)?;
    }

    canvas.present();
    Ok(())
}
