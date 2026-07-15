// ─────────────────────────────────────────────────────────────────────────────
// TODO — Person A implements: is_ahead(), update_turning_direction()
// TODO — Person B implements: draw_intersection_lines(), draw_corrected_lights(),
//                             handle_input()
// ─────────────────────────────────────────────────────────────────────────────

use std::process::exit;
use macroquad::prelude::*;
use crate::types::*;

/// [Person B] Draw the 6 white lane-divider lines (3 vertical + 3 horizontal).
pub fn draw_intersection_lines(_center: Vec2) {
    todo!("Person B implements this")
}

/// [Person B] Draw 4 traffic light squares (30×30px). GREEN if active, RED otherwise.
pub fn draw_corrected_lights(_center: Vec2, _active: Option<Origin>) {
    todo!("Person B implements this")
}

/// [Person A] Returns true if `other` is directly ahead of `pos` in the direction of `speed`.
pub fn is_ahead(_pos: Vec2, _speed: Vec2, _other: Vec2) -> bool {
    todo!("Person A implements this")
}

/// [Person A] Rotate car speed vector at the correct turn point, mark `turned = true`.
pub fn update_turning_direction(_car: &mut Car, _center: Vec2) {
    todo!("Person A implements this")
}

/// [Person B] Handle keyboard input — spawn cars via arrow keys / R / Esc.
pub fn handle_input(
    _cars: &mut Vec<Car>,
    _center: Vec2,
    _lane_capacity: usize,
    _counts: [usize; 4],
) {
    todo!("Person B implements this")
}
