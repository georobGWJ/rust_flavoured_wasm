// Scan in a particular direction and if another bot is detected, return the
// range. Otherwise return 0
pub fn scan(angle:i32, resolution: i32) -> i32{
    unsafe { ffi::scan(angle, resolution) }
}

// Fire the cannon at a given degree and range (max 700 m). Retutn 0 if no
// shot fired or 1 if a shot was fired.
pub fn cannon(angle: i32, range: i32) -> i32{
    unsafe { ffi::cannon(angle, range) }
}

// Move the bot in a given direction at a given speed percentage
pub fn drive(angle: i32, speed: i32) -> i32{
    unsafe { ffi::drive(angle, speed) }
}

// Return the percentage of damage currently taken by the bot
pub fn damage() -> i32{
    unsafe { ffi::damage() }
}

// Return the percentage speed of the bot 
pub fn speed() -> i32{
    unsafe { ffi::speed() }
}

// Return the current x coordinate of the bot
pub fn loc_x() -> i32{
    unsafe { ffi::loc_x() }
}

// Return the current y coordinate of the bot
pub fn loc_y() -> i32{
    unsafe { ffi::loc_y() }
}

// Return a random number between 0 and a limit (max 32,767)
pub fn rand(limit: i32) -> i32{
    unsafe { ffi::rand(limit) }
}

// Return the square root of a number, coerced to positive if needed
pub fn wsqrt(number: i32) -> i32{
    unsafe { ffi::wsqrt(number) }
}

// Trig sin() renamed to avoid namespace pollution
pub fn wsin(degree: i32) -> i32{
    unsafe { ffi::wsin(degree) }
}

// Trig cos() renamed to avoid namespace pollution
pub fn wcos(degree: i32) -> i32{
    unsafe { ffi::wcos(degree) }
}

// Trig tan() renamed to avoid namespace pollution
pub fn wtan(degree: i32) -> i32{
    unsafe { ffi::wtan(degree) }
}

// Trig arctan() renamed to avoid namespace pollution
pub fn watan(degree: i32) -> i32{
    unsafe { ffi::watan(degree) }
}

// Determine the direction to a specified x and y
pub fn plot_course(tx: i32, ty: i32) -> i32{
    unsafe { ffi::plot_course(tx, ty) }
}

// Utility example for moving to a destination and stopping
// This logic does NOT recover from a collision en route
pub fn go(target_x: i32, target_y:i32) {
    let course = plot_course(target_x, target_y);
    drive(course, 20);
    // At speed 20, it should take 2 ticks from awareness of the target
    // to stop on it
    while (target_x - loc_x()).abs() > 40 &&
          (target_y - loc_y()).abs() > 40 &&
           speed() > 0 {
        // wait till we get to the target
    }

    drive(course, 0); // turn off engine
    while speed() > 0 {
        // steady on until we stop
    }
}   

// Counterclockwise azimuths to match original Crobots logic
pub const ANGLE_EAST: i32 = 0;
pub const ANGLE_NORTH: i32 = 90;
pub const ANGLE_WEST: i32 = 180;
pub const ANGLE_SOUTH: i32 = 270;

// Other constants bots can use
pub const MAX_X: u32 = 1000;
pub const MAX_Y: u32 = 1000;

pub const DAMAGE_COLLISION: i32 = 2;
pub const DAMAGE_DIRECTHIT: i32 = 10;
pub const DAMAGE_NEARHIT: i32 = 5;
pub const DAMAGE_FAR_HIT: i32 = 3;

pub const BLAST_RADIUS: i32 = 40;
pub const PROJECTILE_MAX_RANGE: i32 = 200;

mod ffi;

