// This module builds an animated indicator using the concept
// of key frame animation. Each frame has a fixed state

// Define 'lit index' for each of the 16 frames
const KEYFRAMES: [i32; 16] = [0,1,2,3,4,5,6,7, 7,6,5,4,3,2,1,0];

// Exact same imports and exports as the other submodules
extern "C" {
    fn set_led(led_index: i32, r: i32, g: i32, b: i32);
}

#[no_mangle]
pub extern "C" fn sensor_update(_sensor_id: i32, _sensor_value: f64) -> f64 {
    // Nothing should happen since this is a prebaked animation
    0.0
}

#[no_mangle]
pub extern "C" fn apply(frame: i32) {
    let idx = frame % 16;

    // Ensure all LEDs are unlit
    for x in 0..8 {
        unsafe {
            set_led(x, 0, 0, 0);
        }
    }
    unsafe {
        // Turn on the light for the current key frame
        set_led(KEYFRAMES[idx as usize], 255, 0, 0);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
