use std::collections::HashMap;
use specs::{World, WorldExt, Builder, Join};
use vector2d::Vector2D;

use crate::components;

const ROTATION_SPEED: f64 = 1.5;
const PLAYER_SPEED: f64 = 4.5;

pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {
    // Check status of the game world
    // let mut must_reload_world = false;
    // {
    //     let players = ecs.read_storage::<components::Player>();
    //     if players.join().count() < 1 {
    //         must_reload_world = true;
    //     }
    // }

    // if must_reload_world {
    //     ecs.delete_all();
    //     load_world(ecs);
    // }

    let mut positions = ecs.write_storage::<crate::components::Position>();
    let mut players = ecs.write_storage::<crate::components::Player>();
    let mut renderables = ecs.write_storage::<components::Renderable>();

    for (player, pos, renderable) in (&mut players, &mut positions, &mut renderables).join() {
        if crate::utils::is_key_pressed(&key_manager, "D") {
            pos.rot += ROTATION_SPEED;
        }
        if crate::utils::is_key_pressed(&key_manager, "A") {
            pos.rot -= ROTATION_SPEED;            
        }

        update_movement(pos, player);
        if crate::utils::is_key_pressed(&key_manager, "W") {
            let radians = pos.rot.to_radians();
            
            let move_x = PLAYER_SPEED * radians.sin();
            let move_y = PLAYER_SPEED * radians.cos();
            let move_vec = Vector2D::<f64>::new(move_x, move_y);

            player.impulse += move_vec;
        }

        if pos.rot > 360.0 {
            pos.rot -= 360.0;
        }
        if pos.rot < 360.0 {
            pos.rot += 360.0;
        }

        if pos.x > crate::SCREEN_WIDTH.into() {
            pos.x -= crate::SCREEN_WIDTH as f64;
        }
        if pos.x < 0.0 {
            pos.x += crate::SCREEN_WIDTH as f64;
        }
        if pos.y > crate::SCREEN_HEIGHT.into() {
            pos.y -= crate::SCREEN_HEIGHT as f64;
        }
        if pos.y < 0.0 {
            pos.y += crate::SCREEN_HEIGHT as f64;
        }

        //update the graphic to reflect the rotation
        renderable.rot = pos.rot;
    }

}

const FRICTION: f64 = 0.90;
const MAX_SPEED: f64 = 3.5; 

pub fn update_movement(pos: &mut crate::components::Position, player: &mut crate::components::Player) {
    player.cur_speed *= FRICTION;

    player.cur_speed += player.impulse;
    if player.cur_speed.length() > MAX_SPEED {
        player.cur_speed = player.cur_speed.normalise();
        player.cur_speed = player.cur_speed * MAX_SPEED;
    }

    pos.x += player.cur_speed.x;
    pos.y -= player.cur_speed.y;

    player.impulse = vector2d::Vector2D::new(0.0,0.0);
}

pub fn load_world( ecs: &mut World ) {
    ecs.create_entity()
        .with(crate::components::Position{ x: 400.0, y: 400.0, rot: 0.0})
        .with(crate::components::Renderable {
            tex_name: String::from("img/intersection_road_test.png"),
            i_w: 800,
            i_h: 800,
            o_w: 800,
            o_h: 800,
            frame: 0,
            total_frames: 1,
            rot: 0.0,
        })
        .build();

    ecs.create_entity()
        .with(crate::components::Position{ x: 400.0, y: 400.0, rot: 0.0})
        .with(crate::components::Renderable{
            tex_name: String::from("img/car2_test.png"),
            i_w: 50,
            i_h: 100,
            o_w: 50,
            o_h: 100,
            frame: 0,
            total_frames: 1,
            rot: 0.0,
        })
        .with(crate::components::Player{
            impulse: vector2d::Vector2D::new(0.0,0.0),
            cur_speed: vector2d::Vector2D::new(0.0,0.0),
        })
        .build();

    ecs.create_entity()
        .with(crate::components::Position{ x: 400.0, y: 235.0, rot: 45.0 })
        .with(crate::components::Renderable {
            tex_name: String::from("img/asteroid.png"),
            i_w: 100,
            i_h: 100,
            o_w: 50,
            o_h: 50,
            frame: 0,
            total_frames: 1,
            rot: 0.0,
        })
        .with(crate::components::Asteroid {
            speed: 2.5,
            rot_speed: 0.5
        })
        .build();
}