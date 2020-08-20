use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;

use std::env;
use std::time::Duration;

mod maze;
use maze::Maze;

fn maze_draw(
    maze: &Maze,
    canvas: &mut Canvas<sdl2::video::Window>,
    win_width: i32,
    win_height: i32,
) {
    let width = maze.get_width() as i32;
    let height = maze.get_height() as i32;

    let tile_width = win_width / width;
    let tile_height = win_height / height;

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let mut x = 0;

    while x < width {
        let mut y = 0;
        while y < height {
            if maze.has_north(x as usize, y as usize) {
                canvas
                    .draw_line(
                        Point::new(x * tile_width, y * tile_height),
                        Point::new(x * tile_width + tile_width, y * tile_height),
                    )
                    .unwrap();
            }
            if maze.has_south(x as usize, y as usize) {
                canvas
                    .draw_line(
                        Point::new(x * tile_width, y * tile_height + tile_height - 1),
                        Point::new(
                            x * tile_width + tile_width,
                            y * tile_height + tile_height - 1,
                        ),
                    )
                    .unwrap();
            }
            if maze.has_west(x as usize, y as usize) {
                canvas
                    .draw_line(
                        Point::new(x * tile_width, y * tile_height),
                        Point::new(x * tile_width, y * tile_height + tile_height - 1),
                    )
                    .unwrap();
            }
            if maze.has_east(x as usize, y as usize) {
                canvas
                    .draw_line(
                        Point::new(x * tile_width + tile_width - 1, y * tile_height),
                        Point::new(
                            x * tile_width + tile_width - 1,
                            y * tile_height + tile_height - 1,
                        ),
                    )
                    .unwrap();
            }
            y += 1;
        }
        x += 1;
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let mut maze = Maze::new(
        args[1].trim().parse().unwrap(),
        args[2].trim().parse().unwrap(),
    );
    maze.recursive_backtracker();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let win_width = args[3].trim().parse().unwrap();
    let win_height = args[4].trim().parse().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo", win_width, win_height)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    maze_draw(&maze, &mut canvas, win_width as i32, win_height as i32);
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
