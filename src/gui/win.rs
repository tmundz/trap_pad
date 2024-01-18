extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn win_init() -> Result<(), String> {
    // init SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    //create main window
    let window = video_subsystem
        .window("trap_pad", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    // Main loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                // Handle other events here
                _ => {}
            }
        }

        // Update the application state

        // Render the current state
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        // Draw your application's contents here
        canvas.present();

        // Sleep to control the loop speed, if necessary
        ::std::thread::sleep(Duration::from_millis(16)); // Approximately 60 FPS
    }

    Ok(())
}
