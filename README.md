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

- One lane has green at a time; the other three are red.
- When it's time to switch, green goes to whichever waiting lane has the **most cars queued**. The lane that was just cut off is skipped once (so it can't hog green forever), unless no other lane is waiting.
- A green lane holds for at least `MIN_GREEN_TIME` (0.1s, avoids flicker) and at most `MAX_GREEN_TIME` (6s, so one lane can't starve the others). Hitting the max cuts that lane's green immediately so no *new* cars enter — the light then waits for the intersection to actually empty before picking the next lane.
- Lane capacity: `floor(lane_length / (vehicle_length + safety_gap))` = **8 vehicles**. A lane stops accepting new spawns once it's full, which is what keeps congestion bounded (the Dynamic Congestion Rule from the subject).

## How a car crosses (the `in_intersection` flag)

Each car has a `turned` flag (has it finished its left/right pivot) and an `in_intersection` flag (is it currently exempt from the traffic light). The second one is the key idea:

1. While approaching, a car is gated by the light as normal: red means stop.
2. The moment a car is waved through on green, it's marked `in_intersection = true` and from then on **ignores the light completely** for the rest of its trip — exactly like a real driver who entered on green doesn't slam the brakes because the light changed a second later.
3. `in_intersection` is cleared once the car has driven all the way through and clear of the intersection.
4. The light is only allowed to switch lanes when **no car is currently `in_intersection`** — i.e. the intersection is genuinely empty, not just "no car exactly in the middle square." That distinction matters: without it, a car that's already committed to crossing could still be caught mid-crossing by a newly-green lane's traffic.

## Collision Avoidance

- Every car only ever drives along a single compass direction at a time (never diagonally, even mid-turn — a turn is an instant pivot to a new direction), so "is there a car in my way" is just: is there another car within **40 px**, in my lane, further along in the direction I'm driving? No angle math needed.
- If such a car exists, the car behind it simply doesn't move that frame — that's the following/safety-distance behaviour.
- Vehicles stop at a red light and resume automatically once it turns green (see above).

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
