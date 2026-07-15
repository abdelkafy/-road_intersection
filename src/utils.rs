use std::process::exit;
use macroquad::prelude::*;
use ::rand::seq::SliceRandom;
use crate::types::*;

const LANE_OFFSET: f32 = 20.0; 
const ROAD_HALF:   f32 = 40.0; 
const CAR_SPEED:   f32 = 2.0;

const COLOR_STRAIGHT: Color = GREEN;
const COLOR_RIGHT:    Color = YELLOW;
const COLOR_LEFT:     Color = RED;

pub fn draw_intersection_lines(center: Vec2) {
    let cx = center.x;
    let cy = center.y;
    let w  = screen_width();
    let h  = screen_height();

    for &x in &[cx - ROAD_HALF, cx, cx + ROAD_HALF] {
        draw_line(x, 0.0,            x, cy - ROAD_HALF, 1.5, WHITE);
        draw_line(x, cy + ROAD_HALF, x, h,              1.5, WHITE);
    }

    for &y in &[cy - ROAD_HALF, cy, cy + ROAD_HALF] {
        draw_line(0.0,            y, cx - ROAD_HALF, y, 1.5, WHITE);
        draw_line(cx + ROAD_HALF, y, w,              y, 1.5, WHITE);
    }

    draw_line(cx - ROAD_HALF, cy - ROAD_HALF, cx,             cy - ROAD_HALF, 3.0, WHITE); // North
    draw_line(cx,             cy + ROAD_HALF, cx + ROAD_HALF,  cy + ROAD_HALF, 3.0, WHITE); // South
    draw_line(cx - ROAD_HALF, cy,             cx - ROAD_HALF,  cy + ROAD_HALF, 3.0, WHITE); // East
    draw_line(cx + ROAD_HALF, cy - ROAD_HALF, cx + ROAD_HALF,  cy,            3.0, WHITE); // West
}

pub fn draw_corrected_lights(center: Vec2, active: Option<Origin>) {
    let cx = center.x;
    let cy = center.y;
    let lights = [
        (Origin::North, vec2(cx - ROAD_HALF - 20.0, cy - ROAD_HALF - 20.0)), // top-left  grass
        (Origin::West,  vec2(cx + ROAD_HALF + 20.0, cy - ROAD_HALF - 20.0)), // top-right grass
        (Origin::East,  vec2(cx - ROAD_HALF - 20.0, cy + ROAD_HALF + 20.0)), // bot-left  grass
        (Origin::South, vec2(cx + ROAD_HALF + 20.0, cy + ROAD_HALF + 20.0)), // bot-right grass
    ];
    for (origin, pos) in lights {
        let color = if active == Some(origin) { GREEN } else { RED };
        draw_rectangle(pos.x - 15.0, pos.y - 15.0, 30.0, 30.0, color);
        draw_rectangle_lines(pos.x - 15.0, pos.y - 15.0, 30.0, 30.0, 1.5, BLACK);
    }
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
    cars: &mut Vec<Car>,
    center: Vec2,
    lane_capacity: usize,
    counts: [usize; 4],
) {
    if is_key_pressed(KeyCode::Escape) { exit(0); }

    fn random_route() -> Route {
        match ::rand::random::<u8>() % 3 {
            0 => Route::Straight,
            1 => Route::Right,
            _ => Route::Left,
        }
    }

    fn route_color(route: Route) -> Color {
        match route {
            Route::Straight => COLOR_STRAIGHT,
            Route::Right    => COLOR_RIGHT,
            Route::Left     => COLOR_LEFT,
        }
    }

    fn spawn(cars: &mut Vec<Car>, center: Vec2, origin: Origin) {
        let route = random_route();
        let color = route_color(route);
        let cx = center.x;
        let cy = center.y;
        let spawn_gap = 20.0 + 25.0 + 5.0; // vehicle_length + safety_gap + buffer

        let (pos, speed) = match origin {
            Origin::North => {
                let tail_y = cars.iter()
                    .filter(|c| c.origin == Origin::North && !c.turned)
                    .map(|c| c.pos.y)
                    .fold(f32::INFINITY, f32::min);
                let y = if tail_y.is_finite() { tail_y - spawn_gap } else { 0.0 };
                (vec2(cx - LANE_OFFSET, y.min(-30.0)), vec2(0.0, CAR_SPEED))
            }
            Origin::South => {
                let tail_y = cars.iter()
                    .filter(|c| c.origin == Origin::South && !c.turned)
                    .map(|c| c.pos.y)
                    .fold(f32::NEG_INFINITY, f32::max);
                let y = if tail_y.is_finite() { tail_y + spawn_gap } else { screen_height() };
                (vec2(cx + LANE_OFFSET, y.max(screen_height() + 30.0)), vec2(0.0, -CAR_SPEED))
            }
            Origin::East => {
                let tail_x = cars.iter()
                    .filter(|c| c.origin == Origin::East && !c.turned)
                    .map(|c| c.pos.x)
                    .fold(f32::INFINITY, f32::min);
                let x = if tail_x.is_finite() { tail_x - spawn_gap } else { 0.0 };
                (vec2(x.min(-30.0), cy + LANE_OFFSET), vec2(CAR_SPEED, 0.0))
            }
            Origin::West => {
                let tail_x = cars.iter()
                    .filter(|c| c.origin == Origin::West && !c.turned)
                    .map(|c| c.pos.x)
                    .fold(f32::NEG_INFINITY, f32::max);
                let x = if tail_x.is_finite() { tail_x + spawn_gap } else { screen_width() };
                (vec2(x.max(screen_width() + 30.0), cy - LANE_OFFSET), vec2(-CAR_SPEED, 0.0))
            }
        };
        cars.push(Car { pos, speed, origin, route, color, turned: false });
    }

    if is_key_pressed(KeyCode::Up)    && counts[1] < lane_capacity { spawn(cars, center, Origin::South); }
    if is_key_pressed(KeyCode::Down)  && counts[0] < lane_capacity { spawn(cars, center, Origin::North); }
    if is_key_pressed(KeyCode::Right) && counts[2] < lane_capacity { spawn(cars, center, Origin::East);  }
    if is_key_pressed(KeyCode::Left)  && counts[3] < lane_capacity { spawn(cars, center, Origin::West);  }

    if is_key_pressed(KeyCode::R) {
        let pairs = [
            (Origin::North, counts[0]),
            (Origin::South, counts[1]),
            (Origin::East,  counts[2]),
            (Origin::West,  counts[3]),
        ];
        let available: Vec<Origin> = pairs.iter()
            .filter(|(_, c)| *c < lane_capacity)
            .map(|(o, _)| *o)
            .collect();
        if let Some(&origin) = available.choose(&mut ::rand::thread_rng()) {
            spawn(cars, center, origin);
        }
    }
}