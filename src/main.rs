mod types;
mod utils;

use macroquad::prelude::*;
use std::collections::HashMap;
use types::*;
use utils::*;

fn road_intersection() -> Conf {
    Conf {
        window_title: "road_intersection".to_string(),
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(road_intersection)]
async fn main() {
    let mut cars: Vec<Car> = Vec::new();
    let mut active_green: Option<Origin> = None;
    let mut green_timer = 0.0_f32;

    const MIN_GREEN_TIME: f32 = 0.1;
    const CENTER_HALF: f32 = 55.0;

    let lane_length = 400.0_f32;
    let vehicle_length = 20.0_f32;
    let safety_gap = 25.0_f32;
    let lane_capacity = (lane_length / (vehicle_length + safety_gap)) as usize;

    loop {
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);

        let n_c = cars.iter().filter(|c| c.origin == Origin::North && !c.turned).count();
        let s_c = cars.iter().filter(|c| c.origin == Origin::South && !c.turned).count();
        let e_c = cars.iter().filter(|c| c.origin == Origin::East  && !c.turned).count();
        let w_c = cars.iter().filter(|c| c.origin == Origin::West  && !c.turned).count();

        handle_input(&mut cars, center, lane_capacity, [n_c, s_c, e_c, w_c]);


        let ratios = HashMap::from([
            (Origin::North, n_c as f32 / lane_capacity as f32),
            (Origin::South, s_c as f32 / lane_capacity as f32),
            (Origin::East,  e_c as f32 / lane_capacity as f32),
            (Origin::West,  w_c as f32 / lane_capacity as f32),
        ]);

        let center_count = cars
            .iter()
            .filter(|c| {
                (c.pos.x - center.x).abs() < CENTER_HALF
                    && (c.pos.y - center.y).abs() < CENTER_HALF
            })
            .count();

        let center_empty = center_count == 0;

        let dt = get_frame_time();
        green_timer += dt;

        let should_switch = match active_green {
            None    => true,
            Some(_) => center_empty && green_timer >= MIN_GREEN_TIME,
        };

        if should_switch {
            let mut best_lane  = None;
            let mut best_score = -1.0;

            for lane in [Origin::North, Origin::South, Origin::East, Origin::West] {
                let cars_in_lane = match lane {
                    Origin::North => n_c,
                    Origin::South => s_c,
                    Origin::East  => e_c,
                    Origin::West  => w_c,
                };

                if cars_in_lane == 0 {
                    continue;
                }

                let score = ratios[&lane];

                if score > best_score {
                    best_score = score;
                    best_lane  = Some(lane);
                }
            }

            if best_lane.is_some() {
                active_green = best_lane;
                green_timer  = 0.0;
            }
        }

        clear_background(BLACK);
        draw_intersection_lines(center);
        draw_corrected_lights(center, active_green);

        let mut i = 0;
        while i < cars.len() {
            let mut can_move = true;
            let car_origin = cars[i].origin;

            if !cars[i].turned {
                let is_at_stop = match car_origin {
                    Origin::South => {
                        cars[i].pos.y > center.y + 40.0 && cars[i].pos.y < center.y + 60.0
                    }
                    Origin::North => {
                        cars[i].pos.y < center.y - 40.0 && cars[i].pos.y > center.y - 60.0
                    }
                    Origin::East => {
                        cars[i].pos.x < center.x - 40.0 && cars[i].pos.x > center.x - 60.0
                    }
                    Origin::West => {
                        cars[i].pos.x > center.x + 40.0 && cars[i].pos.x < center.x + 60.0
                    }
                };

                if is_at_stop && Some(car_origin) != active_green {
                    can_move = false;
                }
            }

            if can_move {
                for j in 0..cars.len() {
                    if i != j
                        && cars[i].pos.distance(cars[j].pos) < 40.0
                        && cars[i].speed.dot(cars[j].speed) > 0.0
                        && is_ahead(cars[i].pos, cars[i].speed, cars[j].pos)
                    {
                        can_move = false;
                        break;
                    }
                }
            }

            if can_move {
                let sv = cars[i].speed;
                cars[i].pos += sv;
                if !cars[i].turned {
                    update_turning_direction(&mut cars[i], center);
                }
            }

            draw_rectangle(cars[i].pos.x - 10.0, cars[i].pos.y - 10.0, 20.0, 20.0, cars[i].color);
            draw_rectangle_lines(cars[i].pos.x - 10.0, cars[i].pos.y - 10.0, 20.0, 20.0, 1.5, BLACK);

            if cars[i].pos.x < -100.0
                || cars[i].pos.x > screen_width()  + 100.0
                || cars[i].pos.y < -100.0
                || cars[i].pos.y > screen_height() + 100.0
            {
                cars.remove(i);
            } else {
                i += 1;
            }
        }

        // HUD
        draw_text(&format!("Cars: {}", cars.len()), 10.0, 20.0, 18.0, WHITE);
        draw_text("GREEN=Straight  YELLOW=Right  RED=Left", 10.0, 42.0, 16.0, WHITE);
        draw_text("Up=South  Down=North  Right=West  Left=East  R=Random  Esc=Quit", 10.0, 62.0, 16.0, WHITE);

        next_frame().await
    }
}