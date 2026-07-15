use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection - Step 1".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        let center = vec2(
            screen_width() / 2.0,
            screen_height() / 2.0,
        );

        clear_background(Color::from_rgba(30, 30, 30, 255));

        draw_roads(center);

        next_frame().await;
    }
}

fn draw_roads(center: Vec2) {
    let road_width = 180.0;

    // Vertical road
    draw_rectangle(
        center.x - road_width / 2.0,
        0.0,
        road_width,
        screen_height(),
        DARKGRAY,
    );

    // Horizontal road
    draw_rectangle(
        0.0,
        center.y - road_width / 2.0,
        screen_width(),
        road_width,
        DARKGRAY,
    );


    // Lane lines
    let line_color = YELLOW;

    // Vertical lane divider
    draw_line(
        center.x,
        0.0,
        center.x,
        screen_height(),
        2.0,
        line_color,
    );

    // Horizontal lane divider
    draw_line(
        0.0,
        center.y,
        screen_width(),
        center.y,
        2.0,
        line_color,
    );


    // Intersection square
    draw_rectangle_lines(
        center.x - 90.0,
        center.y - 90.0,
        180.0,
        180.0,
        3.0,
        WHITE,
    );
}