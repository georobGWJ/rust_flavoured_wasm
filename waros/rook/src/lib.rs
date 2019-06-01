// Inspired by https://github.com/tpoindex/crobots/blob/master/src/rook.r

// This 'bot' moves to the center of the field and then patrols from
// east to west, scanning N, S, E, and W for targets. If it is hit while
// scanning, it will change direction

// Ignores incoming fire while moving to the center of the field

extern crate warsdk;
use warsdk::*;

// Note that the bot can maintain it's own internal mutable state.
struct State {
    course: i32,
}

#[no_mangle]
pub extern "C" fn botinit() -> i32 {
    go(500, 500);

    let mut state = State { course: 0 };

    loop {
        look(ANGLE_EAST,  &mut state);
        look(ANGLE_NORTH, &mut state);
        look(ANGLE_WEST,  &mut state);
        look(ANGLE_SOUTH, &mut state);

        if loc_x() > BOUND_X_MAX || loc_x() < BOUND_X_MIN {
            reverse(&mut state);
        }

        if speed() == 0 {
            // Hit something and stopped
            reverse(&mut state);
        }
    }
}

fn look(angle: i32, state: &mut State) {
    let mut range = scan(angle, 2);

    // Fire at targets in range as long as they're in range and alive
    while range > 0 && range < PROJECTILE_MAX_RANGE as i32 {
        if speed() > 0 {
            drive(state.course, 0);
        }

        if range > BLAST_RADIUS {
            // Ensure we're not in range of the explosion
            cannon(angle, range);
        }
        range = scan(angle, 2);
    }
}

fn reverse(state: &mut State) {
    if state.course == ANGLE_EAST {
        state.course = ANGLE_WEST;
    } else {
        state.course = ANGLE_EAST;
    }
}

const BOUND_X_MIN: i32 = 80;
const BOUND_X_MAX: i32 = 920;