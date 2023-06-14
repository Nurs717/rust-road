use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::{Rect, Point};
use sdl2::video::WindowContext;

use specs::{World, WorldExt, Join, DispatcherBuilder};

use std::time::Duration;
use std::collections::HashMap;

pub mod texture_manager;
pub mod utils;
pub mod components;
pub mod game;
pub mod asteroid;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

fn render(canvas: &mut WindowCanvas, texture_manager: &mut texture_manager::TextureManager<WindowContext>, ecs: &World) -> Result<(), String> {
    let color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(color);
    canvas.clear();

    let positions = ecs.read_storage::<components::Position>();
    let renderables = ecs.read_storage::<components::Renderable>();

    for (renderable, pos) in (&renderables, &positions).join() {
        let src = Rect::new(0,0, renderable.i_w, renderable.i_h);
        let x: i32 = pos.x as i32;
        let y: i32 = pos.y as i32;
        let dest = Rect::new(x - ((renderable.o_w/2) as i32), y - ((renderable.o_h/2) as i32), renderable.o_w, renderable.o_h);
    
        let center = Point::new((renderable.o_w/2) as i32, (renderable.o_h/2) as i32);
        let texture = texture_manager.load(&renderable.tex_name)?;
        canvas.copy_ex(
            &texture, 
            src, // source rect
            dest, // destination rect
            renderable.rot, // Angle of rotation in degrees
            center, // center of image
            false, // flip horizontal
            false, // flip vertical
        )?;
    }

    canvas.present();

    Ok(())
}

struct State { ecs: World }

fn main() -> Result<(), String> {
    println!("Starting smart road!!!");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Smart Road", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not init the video subsystem");

    let mut canvas = window.into_canvas().build()
                                                            .expect("failed to init canvas");

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = texture_manager::TextureManager::new(&texture_creator);

    //Load Image
    texture_manager.load("img/car2_test.png")?;
    texture_manager.load("img/asteroid.png")?;
    texture_manager.load("img/intersection_road_test.png")?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut key_manager: HashMap<String, bool> = HashMap::new();

    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Renderable>();
    gs.ecs.register::<components::Player>();
    gs.ecs.register::<components::Asteroid>();

    let mut dispatcher = DispatcherBuilder::new()
                                            .with(asteroid::AsteroidMover, "asteroid_mover", &[])
                                            .with(asteroid::AsteroidCollider, "asteroid_collider", &[])
                                            .build();

    game::load_world(&mut gs.ecs);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        None => {}
                        Some(key) => {
                            utils::key_down(&mut key_manager, key.to_string())
                        }
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        None => {}
                        Some(key) => {
                            utils::key_up(&mut key_manager, key.to_string())
                        }
                    }
                },
                _ => {}
            }
        }
        game::update(&mut gs.ecs, &mut key_manager);
        dispatcher.dispatch(&gs.ecs);
        gs.ecs.maintain();
        render(&mut canvas, &mut texture_manager, &gs.ecs)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    }

    Ok(())
}

fn _render_background(canvas: &mut WindowCanvas, texture_manager: &mut texture_manager::TextureManager<WindowContext>) -> Result<(), String> {
    let texture = texture_manager.load("img/intersection_road_test.png")?;
    canvas.copy(
        &texture,
        None,
        None,
    )?;
    canvas.present();
    Ok(())
}