# road_intersection

A traffic intersection simulation built with Rust and [macroquad](https://macroquad.rs/).

## Overview

Two roads cross at a single intersection. Each road has one lane in each direction. A dynamic traffic light system controls which lane has right of way, preventing collisions while minimising congestion.

## Controls

| Key | Action |
|-----|--------|
| `↑` Up | Spawn a vehicle from the **south** (moves north) |
| `↓` Down | Spawn a vehicle from the **north** (moves south) |
| `→` Right | Spawn a vehicle from the **west** (moves east) |
| `←` Left | Spawn a vehicle from the **east** (moves west) |
| `R` | Spawn a vehicle from a **random** direction |
| `Esc` | Exit the simulation |

Holding a key does not spam vehicles — each press spawns at most one vehicle, and spawning is blocked when the lane is at capacity.

## Vehicle Colors

Car color indicates the route the vehicle will follow:

| Color | Route |
|-------|-------|
| **Green** | Straight |
| **Yellow** | Turn right |
| **Red** | Turn left |

## Traffic Light System

- One lane receives a green light at a time.
- The lane with the **highest congestion ratio** (vehicles / capacity) is given priority.
- The light switches only when the intersection center is clear and a minimum hold time has elapsed.
- Lane capacity is calculated as: `floor(lane_length / (vehicle_length + safety_gap))` = **8 vehicles**.

## Collision Avoidance

- Vehicles maintain a **40 px** minimum center-to-center distance from the vehicle ahead.
- Only vehicles travelling in the **same direction** can block each other — turned vehicles do not stall the lane they just left.
- Vehicles stop at a red light and resume automatically when the light turns green.

## Running

```bash
cargo run
```

Requires Rust and the system dependencies for macroquad (OpenGL, X11 on Linux).

## Project Structure

```
src/
  main.rs    — game loop, traffic light logic, rendering
  utils.rs   — drawing helpers, collision detection, input handling
  types.rs   — Route, Origin, Car types
```
