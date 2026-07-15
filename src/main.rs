use macroquad::prelude::*;
use std::collections::HashMap;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}


#[derive(Clone, Copy)]
enum Route {
    Straight,
    Left,
    Right,
}


struct Car {
    pos: Vec2,
    velocity: Vec2,
    direction: Direction,
    route: Route,
    color: Color,
    turned: bool,
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection - Step 4".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}



#[macroquad::main(window_conf)]
async fn main() {

    let mut cars: Vec<Car> = Vec::new();

    let mut green: Option<Direction> = None;

    let mut green_timer = 0.0;


    const MIN_GREEN_TIME:f32 = 2.0;


    loop {

        let center =
            vec2(
                screen_width()/2.0,
                screen_height()/2.0
            );


        handle_spawn(
            &mut cars,
            center
        );



        let counts =
        HashMap::from([
            (
                Direction::North,
                cars.iter()
                .filter(|c| c.direction==Direction::North)
                .count()
            ),

            (
                Direction::South,
                cars.iter()
                .filter(|c| c.direction==Direction::South)
                .count()
            ),

            (
                Direction::East,
                cars.iter()
                .filter(|c| c.direction==Direction::East)
                .count()
            ),

            (
                Direction::West,
                cars.iter()
                .filter(|c| c.direction==Direction::West)
                .count()
            ),
        ]);



        let dt=get_frame_time();

        green_timer+=dt;



        if green.is_none()
            || green_timer>MIN_GREEN_TIME
        {

            let mut best=None;
            let mut max=0;


            for (dir,count) in &counts {

                if *count>max {

                    max=*count;
                    best=Some(*dir);

                }
            }


            if best.is_some() {

                green=best;
                green_timer=0.0;

            }
        }



        for i in 0..cars.len() {


            let mut move_car=true;


            if !cars[i].turned {


                if at_stop(
                    &cars[i],
                    center
                )
                {

                    if Some(cars[i].direction)
                        != green
                    {
                        move_car=false;
                    }
                }
            }



            if move_car {


                for j in 0..cars.len() {


                    if i!=j
                    && cars[i]
                    .pos
                    .distance(cars[j].pos)<40.0
                    {

                        move_car=false;
                    }
                }

            }




            if move_car {

                cars[i].pos +=
                    cars[i].velocity * dt;


                turn_car(
                    &mut cars[i],
                    center
                );
            }

        }



        cars.retain(|c|
            c.pos.x>-100.0
            && c.pos.x<screen_width()+100.0
            && c.pos.y>-100.0
            && c.pos.y<screen_height()+100.0
        );



        clear_background(BLACK);


        draw_roads(center);


        draw_lights(
            center,
            green
        );



        for car in &cars {

            draw_rectangle(
                car.pos.x-10.0,
                car.pos.y-10.0,
                20.0,
                20.0,
                car.color
            );

        }



        draw_text(
            &format!("Cars {}",cars.len()),
            10.0,
            25.0,
            25.0,
            WHITE
        );



        next_frame().await;
    }
}




fn at_stop(
    car:&Car,
    center:Vec2
)->bool{


    match car.direction {


        Direction::North =>
            car.pos.y>center.y-120.0
            &&
            car.pos.y<center.y-60.0,


        Direction::South =>
            car.pos.y<center.y+120.0
            &&
            car.pos.y>center.y+60.0,


        Direction::East =>
            car.pos.x<center.x+120.0
            &&
            car.pos.x>center.x+60.0,


        Direction::West =>
            car.pos.x>center.x-120.0
            &&
            car.pos.x<center.x-60.0,

    }

}




fn turn_car(
    car:&mut Car,
    center:Vec2
){


    if car.turned {
        return;
    }


    if car.pos.distance(center)<40.0 {

        match car.route {


            Route::Left => {

                let old=car.velocity;

                car.velocity=
                    vec2(
                        -old.y,
                        old.x
                    );
            }



            Route::Right => {

                let old=car.velocity;

                car.velocity=
                    vec2(
                        old.y,
                        -old.x
                    );
            }



            Route::Straight=>{}

        }


        car.turned=true;

    }

}





fn handle_spawn(
    cars:&mut Vec<Car>,
    center:Vec2
){

    let speed=120.0;


    if is_key_pressed(KeyCode::Up){

        cars.push(
            Car{
                pos:vec2(center.x,-20.0),
                velocity:vec2(0.0,speed),
                direction:Direction::North,
                route:Route::Straight,
                color:BLUE,
                turned:false
            }
        );
    }


    if is_key_pressed(KeyCode::Down){

        cars.push(
            Car{
                pos:vec2(center.x,screen_height()+20.0),
                velocity:vec2(0.0,-speed),
                direction:Direction::South,
                route:Route::Straight,
                color:RED,
                turned:false
            }
        );
    }

}





fn draw_lights(
    center:Vec2,
    green:Option<Direction>
){

    let positions=[

        (Direction::North,vec2(center.x,40.0)),

        (Direction::South,vec2(center.x,screen_height()-40.0)),

        (Direction::East,vec2(screen_width()-40.0,center.y)),

        (Direction::West,vec2(40.0,center.y)),

    ];


    for (dir,pos) in positions {


        draw_circle(
            pos.x,
            pos.y,
            12.0,
            if Some(dir)==green
            {
                GREEN
            }
            else
            {
                RED
            }
        );

    }

}



fn draw_roads(center:Vec2){

    draw_rectangle(
        center.x-90.0,
        0.0,
        180.0,
        screen_height(),
        DARKGRAY
    );


    draw_rectangle(
        0.0,
        center.y-90.0,
        screen_width(),
        180.0,
        DARKGRAY
    );
}