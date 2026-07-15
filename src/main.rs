use macroquad::prelude::*;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Car {
    pos: Vec2,
    velocity: Vec2,
    color: Color,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection - Step 2".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cars: Vec<Car> = Vec::new();

    loop {
        let center = vec2(
            screen_width() / 2.0,
            screen_height() / 2.0,
        );

        handle_input(&mut cars, center);

        let dt = get_frame_time();

        for car in cars.iter_mut() {
            car.pos += car.velocity * dt;
        }

        cars.retain(|car| {
            car.pos.x > -50.0
                && car.pos.x < screen_width() + 50.0
                && car.pos.y > -50.0
                && car.pos.y < screen_height() + 50.0
        });


        clear_background(Color::from_rgba(30, 30, 30, 255));

        draw_roads(center);

        for car in &cars {
            draw_rectangle(
                car.pos.x - 10.0,
                car.pos.y - 10.0,
                20.0,
                20.0,
                car.color,
            );
        }


        draw_text(
            &format!("Cars: {}", cars.len()),
            10.0,
            25.0,
            25.0,
            WHITE,
        );

        next_frame().await;
    }
}


fn handle_input(cars: &mut Vec<Car>, center: Vec2) {

    let speed = 120.0;


    if is_key_pressed(KeyCode::Up) {
        cars.push(Car {
            pos: vec2(center.x, -20.0),
            velocity: vec2(0.0, speed),
            color: BLUE,
        });
    }


    if is_key_pressed(KeyCode::Down) {
        cars.push(Car {
            pos: vec2(center.x, screen_height() + 20.0),
            velocity: vec2(0.0, -speed),
            color: RED,
        });
    }


    if is_key_pressed(KeyCode::Left) {
        cars.push(Car {
            pos: vec2(screen_width() + 20.0, center.y),
            velocity: vec2(-speed, 0.0),
            color: GREEN,
        });
    }


    if is_key_pressed(KeyCode::Right) {
        cars.push(Car {
            pos: vec2(-20.0, center.y),
            velocity: vec2(speed, 0.0),
            color: YELLOW,
        });
    }
}


fn draw_roads(center: Vec2) {

    let road_width = 180.0;


    draw_rectangle(
        center.x - road_width / 2.0,
        0.0,
        road_width,
        screen_height(),
        DARKGRAY,
    );


    draw_rectangle(
        0.0,
        center.y - road_width / 2.0,
        screen_width(),
        road_width,
        DARKGRAY,
    );


    draw_line(
        center.x,
        0.0,
        center.x,
        screen_height(),
        2.0,
        YELLOW,
    );


    draw_line(
        0.0,
        center.y,
        screen_width(),
        center.y,
        2.0,
        YELLOW,
    );


    draw_rectangle_lines(
        center.x - 90.0,
        center.y - 90.0,
        180.0,
        180.0,
        3.0,
        WHITE,
    );
}