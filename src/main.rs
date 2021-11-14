#![allow(dead_code)]

mod asteroids;
mod circles;
mod collision;
mod console;
mod utils;
mod widget;

use std::path::Path;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() -> () {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = 800;
    let window_height = 600;

    let window = video_subsystem
        .window("RAsteroids", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();
    canvas.clear();


    let texture_creator = canvas.texture_creator();

    let resource_path = Path::new("resources");
    let image_resources = asteroids::ImageResources::from_dir(resource_path, &texture_creator);


    let frame_per_second_target = 60;
    let _milliseconds_per_frame = 1000.0 / frame_per_second_target as f32;

    // need some sort of stateful item for what has focus.
    // need to then pass the event to w/e item has current focuse
    // then each item has a sort of "back out" option.

    // w/e widget has focus is the current "top" widget.
    // widget_stack.push(Box::new(Console::new()));

    // let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    // let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // p.push("lazy.ttf");

    let mut game_state = asteroids::game_init();

    let game_input = asteroids::GameInput {
        rotation: 1.0,
        shoot: true,
        thrusters: false,
    };

    // hold the app and wait for user to quit.
    'holding_loop: loop {
        canvas.clear();

        game_state = asteroids::game_update(game_state, 1.0, &game_input);

        asteroids::game_sdl2_render(&game_state, &mut canvas, &image_resources);
        canvas.present();

        std::thread::sleep(std::time::Duration::from_millis(10));

        // event processing which is sent directly to the top layer widget.
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'holding_loop,
                Event::KeyUp {
                    timestamp,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod,
                    repeat,
                } => {
                    println!(
                        "Up timestamp: {}, repeat: {}, keycode: {}, keymode: {}",
                        timestamp,
                        repeat,
                        keycode.unwrap(),
                        keymod
                    );

                    match keycode {
                        Some(Keycode::Backquote) => {
                            // todo: display / push the console onto the stack.
                        }
                        _ => (),
                    };
                }
                Event::KeyDown {
                    timestamp,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod,
                    repeat,
                } => {
                    println!(
                        "Down timestamp: {}, repeat: {}, keycode: {}, keymode: {}",
                        timestamp,
                        repeat,
                        keycode.unwrap(),
                        keymod
                    );
                    match keycode {
                        Some(Keycode::Space) => {
                            canvas.clear();
                        }
                        _ => (),
                    }
                }

                Event::MultiGesture { .. } => {
                    println!("Got a multigesture");
                }
                Event::MouseButtonDown { .. } => (),
                _ => {
                    // println!("Got a random key press");
                }
            }
        }
    }
}
