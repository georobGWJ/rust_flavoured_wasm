// The Battery Indicator module gets the percentage
// of battery charge and uses this value to contol the 
// color of a group of 8 LEDs

// A Tuple struct to hold the R, G, B color codes
#[derive(Clone, Debug, PartialEq)]
struct LedColor(i32, i32, i32);

// A constant that links a unique id to a particular sensor
const SENSOR_BATTERY: i32 = 20;

const OFF:LedColor    = LedColor(0, 0, 0);
const YELLOW:LedColor = LedColor(255, 255, 0);
const GREEN:LedColor  = LedColor(0, 255, 0);
const RED:LedColor    = LedColor(255, 0, 0);
const PCT_PER_PIXEL: f64 = 12.5_f64;

// Import the set_led() function from our host (loose coupling).
extern "C" {
    fn set_led(led_index: i32, r: i32, g: i32, b: i32);
}

// Expose the sensor_update() and apply() functions to the host.
#[no_mangle]
pub extern "C" fn sensor_update(sensor_id: i32, sensor_value: f64) -> f64 {
    if sensor_id == SENSOR_BATTERY {
        set_leds(get_led_values(sensor_value));
    }
    sensor_value
}

#[no_mangle]
pub extern "C" fn apply(_frame: u32) {
    // Nothing should happen since this is not an animated indicator
}

// Logic to convert a percentage to an array of 8 color codes
fn get_led_values(battery_remaining: f64) -> [LedColor; 8] {
    let mut arr: [LedColor;8] = [OFF, OFF, OFF, OFF, OFF, OFF, OFF, OFF];
    let lit = (battery_remaining / PCT_PER_PIXEL).ceil();

    //  0% - 20%    RED
    // 21% - 50%    YELLOW
    // 51% - 100%   GREEN

    let color = if 0.0 <= battery_remaining && battery_remaining <= 20.0 {
        RED
    } else if battery_remaining > 20.0 && battery_remaining <= 50.0 {
        YELLOW
    } else {
        GREEN
    };

    for idx in 0..lit as usize {
        arr[idx] = color.clone();
    }

    arr
}

// Invoke the unsafe block in a loop to set all LED colors on the host.
fn set_leds(values: [LedColor; 8]) {
    for x in 0..8 {
        let LedColor(r, g, b) = values[x];
        unsafe {
            set_led(x as i32, r, g, b);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{OFF, YELLOW, RED, GREEN, get_led_values};

    #[test]
    fn test_0_percent() {
        assert_eq!(get_led_values(0.0), 
            [OFF, OFF, OFF, OFF, OFF, OFF, OFF, OFF]);
    }

    #[test]
    fn test_15_percent() {
        assert_eq!(get_led_values(15.0), 
            [RED, RED, OFF, OFF, OFF, OFF, OFF, OFF]);
    }

    #[test]
    fn test_49_percent() {
        assert_eq!(get_led_values(49.0), 
            [YELLOW, YELLOW, YELLOW, YELLOW, OFF, OFF, OFF, OFF]);
    }

    #[test]
    fn test_75_percent() {
        assert_eq!(get_led_values(75.0), 
            [GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, OFF, OFF]);
    }

    #[test]
    fn test_100_percent() {
        assert_eq!(get_led_values(100.0), 
            [GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, GREEN]);
    }

}
