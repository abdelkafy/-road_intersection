use std::process::exit;
use macroquad::prelude::*;
use ::rand::seq::SliceRandom;
use crate::types::*;

const LANE_OFFSET: f32 = 20.0; 
const ROAD_HALF:   f32 = 40.0; // half the total road width
const CAR_SPEED:   f32 = 2.0;

const COLOR_STRAIGHT: Color = GREEN;
const COLOR_RIGHT:    Color = YELLOW;
const COLOR_LEFT:     Color = RED;

pub fn draw_intersection_lines(center: Vec2) {
    todo!("Person B implements this")
}

pub fn draw_corrected_lights(center: Vec2, active: Option<Origin>) {
    todo!("Person B implements this")
}

pub fn is_ahead(pos: Vec2, speed: Vec2, other: Vec2) -> bool {
    let to_other = other - pos;
    let forward = to_other.dot(speed);
    if forward <= 0.0 { return false; }
    let speed_sq = speed.dot(speed);
    if speed_sq < 0.001 { return false; }
    let lateral_sq = to_other.dot(to_other) - (forward * forward) / speed_sq;
    lateral_sq < LANE_OFFSET * LANE_OFFSET
}

pub fn update_turning_direction(car: &mut Car, center: Vec2) {
    let cx = center.x;
    let cy = center.y;

    let reached = match (car.origin, car.route) {
        (Origin::North, Route::Right)    => car.pos.y >= cy - LANE_OFFSET,
        (Origin::North, _)               => car.pos.y >= cy + LANE_OFFSET,
        (Origin::South, Route::Right)    => car.pos.y <= cy + LANE_OFFSET,
        (Origin::South, _)               => car.pos.y <= cy - LANE_OFFSET,
        (Origin::East,  Route::Right)    => car.pos.x >= cx - LANE_OFFSET,
        (Origin::East,  _)               => car.pos.x >= cx + LANE_OFFSET,
        (Origin::West,  Route::Right)    => car.pos.x <= cx + LANE_OFFSET,
        (Origin::West,  _)               => car.pos.x <= cx - LANE_OFFSET,
    };
    if !reached { return; }

    let mag = car.speed.length();

    match (car.origin, car.route) {
        (Origin::North, Route::Right) => { car.speed = vec2(-mag,  0.0); }
        (Origin::North, Route::Left)  => { car.speed = vec2( mag,  0.0); }
        (Origin::South, Route::Right) => { car.speed = vec2( mag,  0.0); }
        (Origin::South, Route::Left)  => { car.speed = vec2(-mag,  0.0); }
        (Origin::East,  Route::Right) => { car.speed = vec2( 0.0,  mag); }
        (Origin::East,  Route::Left)  => { car.speed = vec2( 0.0, -mag); }
        (Origin::West,  Route::Right) => { car.speed = vec2( 0.0, -mag); }
        (Origin::West,  Route::Left)  => { car.speed = vec2( 0.0,  mag); }
        (_, Route::Straight) => {}
    }
    car.turned = true;
}


pub fn handle_input(
    _cars: &mut Vec<Car>,
    _center: Vec2,
    _lane_capacity: usize,
    _counts: [usize; 4],
) {
    todo!("Person B implements this")
}


