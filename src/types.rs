// ─────────────────────────────────────────────
// TODO — Person A implements this file
// Defines: Route enum, Origin enum, Car struct
// ─────────────────────────────────────────────

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Route {
    Right,
    Left,
    Straight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Origin {
    North,
    South,
    West,
    East,
}

pub struct Car {
    pub pos: Vec2,
    pub speed: Vec2,
    pub origin: Origin,
    pub route: Route,
    pub color: Color,
    pub turned: bool,
}
