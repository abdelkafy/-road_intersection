use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Car {
    pos: Vec2,
    velocity: Vec2,
    direction: Direction,
    color: Color,
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection - Step 3".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {

    let mut cars: Vec<Car> = Vec::new();

    let mut green = Direction::North;


    loop {

        let center = vec2(
            screen_width() / 2.0,
            screen_height() / 2.0,
        );


        handle_light_input(&mut green);

        handle_input(&mut cars, center);


        let dt = get_frame_time();


        for car in cars.iter_mut() {

            if can_move(car, green, center) {
                car.pos += car.velocity * dt;
            }
        }


        cars.retain(|car| {
            car.pos.x > -50.0
            && car.pos.x < screen_width() + 50.0
            && car.pos.y > -50.0
            && car.pos.y < screen_height() + 50.0
        });



        clear_background(Color::from_rgba(30,30,30,255));


        draw_roads(center);

        draw_lights(center, green);


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
            "1:N 2:S 3:E 4:W",
            10.0,
            25.0,
            25.0,
            WHITE,
        );


        draw_text(
            &format!("Cars: {}", cars.len()),
            10.0,
            50.0,
            25.0,
            WHITE,
        );


        next_frame().await;
    }
}



fn handle_light_input(green: &mut Direction) {

    if is_key_pressed(KeyCode::Key1) {
        *green = Direction::North;
    }

    if is_key_pressed(KeyCode::Key2) {
        *green = Direction::South;
    }

    if is_key_pressed(KeyCode::Key3) {
        *green = Direction::East;
    }

    if is_key_pressed(KeyCode::Key4) {
        *green = Direction::West;
    }
}



fn can_move(
    car: &Car,
    green: Direction,
    center: Vec2,
) -> bool {


    let stop_distance = 70.0;


    match car.direction {

        Direction::North => {

            if car.pos.y < center.y - stop_distance
                && car.pos.y > center.y - 120.0
            {
                return green == Direction::North;
            }

        }


        Direction::South => {

            if car.pos.y > center.y + stop_distance
                && car.pos.y < center.y + 120.0
            {
                return green == Direction::South;
            }

        }


        Direction::East => {

            if car.pos.x > center.x + stop_distance
                && car.pos.x < center.x + 120.0
            {
                return green == Direction::East;
            }

        }


        Direction::West => {

            if car.pos.x < center.x - stop_distance
                && car.pos.x > center.x - 120.0
            {
                return green == Direction::West;
            }

        }
    }


    true
}




fn handle_input(
    cars: &mut Vec<Car>,
    center: Vec2
) {

    let speed = 120.0;


    if is_key_pressed(KeyCode::Up) {

        cars.push(Car {
            pos: vec2(center.x,-20.0),
            velocity: vec2(0.0,speed),
            direction: Direction::North,
            color: BLUE,
        });

    }



    if is_key_pressed(KeyCode::Down) {

        cars.push(Car {
            pos: vec2(center.x,screen_height()+20.0),
            velocity: vec2(0.0,-speed),
            direction: Direction::South,
            color: RED,
        });

    }



    if is_key_pressed(KeyCode::Right) {

        cars.push(Car {
            pos: vec2(-20.0,center.y),
            velocity: vec2(speed,0.0),
            direction: Direction::West,
            color: GREEN,
        });

    }



    if is_key_pressed(KeyCode::Left) {

        cars.push(Car {
            pos: vec2(screen_width()+20.0,center.y),
            velocity: vec2(-speed,0.0),
            direction: Direction::East,
            color: YELLOW,
        });

    }

}



fn draw_lights(
    center: Vec2,
    green: Direction
) {


    let size = 25.0;


    let lights = [
        (
            Direction::North,
            vec2(center.x,50.0)
        ),
        (
            Direction::South,
            vec2(center.x,screen_height()-50.0)
        ),
        (
            Direction::East,
            vec2(screen_width()-50.0,center.y)
        ),
        (
            Direction::West,
            vec2(50.0,center.y)
        ),
    ];


    for (dir,pos) in lights {


        let color =
            if dir == green {
                GREEN
            } else {
                RED
            };


        draw_rectangle(
            pos.x-size/2.0,
            pos.y-size/2.0,
            size,
            size,
            color,
        );
    }

}



fn draw_roads(center: Vec2) {

    let road_width = 180.0;


    draw_rectangle(
        center.x-road_width/2.0,
        0.0,
        road_width,
        screen_height(),
        DARKGRAY
    );


    draw_rectangle(
        0.0,
        center.y-road_width/2.0,
        screen_width(),
        road_width,
        DARKGRAY
    );


    draw_line(
        center.x,
        0.0,
        center.x,
        screen_height(),
        2.0,
        YELLOW
    );


    draw_line(
        0.0,
        center.y,
        screen_width(),
        center.y,
        2.0,
        YELLOW
    );
}