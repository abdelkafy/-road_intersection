mod types;
mod utils;

use macroquad::prelude::*;
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
    let mut last_green:   Option<Origin> = None;
    let mut green_timer = 0.0_f32;

    const MIN_GREEN_TIME: f32 = 0.1; 
    const MAX_GREEN_TIME: f32 = 6.0; 

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

        let intersection_clear = !cars.iter().any(|c| c.in_intersection);

        let dt = get_frame_time();
        green_timer += dt;

        if active_green.is_some() && green_timer >= MAX_GREEN_TIME {
            last_green   = active_green;
            active_green = None;
        }

        let should_switch = match active_green {
            None    => intersection_clear,
            Some(_) => intersection_clear && green_timer >= MIN_GREEN_TIME,
        };

        if should_switch {
            let counts = [
                (Origin::North, n_c),
                (Origin::South, s_c),
                (Origin::East,  e_c),
                (Origin::West,  w_c),
            ];

            let mut best_lane = None;
            let mut best_count = 0;
            for (lane, count) in counts {
                if Some(lane) == last_green || count == 0 {
                    continue;
                }
                if count > best_count {
                    best_lane = Some(lane);
                    best_count = count;
                }
            }

            active_green = best_lane.or(last_green);
            green_timer  = 0.0;
            last_green   = None;
        }

        clear_background(BLACK);
        draw_intersection_lines(center);
        draw_corrected_lights(center, active_green);

        let mut i = 0;
        while i < cars.len() {
            let mut can_move = apply_traffic_light(&mut cars[i], center, active_green);

            if can_move {
                for j in 0..cars.len() {
                    if i != j && is_blocking(&cars[i], &cars[j]) {
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

        draw_text(&format!("Cars: {}", cars.len()), 10.0, 20.0, 18.0, WHITE);
        draw_text("GREEN=Straight  YELLOW=Right  RED=Left", 10.0, 42.0, 16.0, WHITE);
        draw_text("Up=South  Down=North  Right=West  Left=East  R=Random  Esc=Quit", 10.0, 62.0, 16.0, WHITE);

        next_frame().await
    }
}
