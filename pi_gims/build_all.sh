# Running `cargo build` from the pi_gims root directory will fail since the 
# binary is 64 bit and the wasm isn't, even though they don't need to link 
# directly.
# This script will successfully build the project and move the appropriate 
# indicator.wasm file (in our case the battery_indicator) to a place the binary
# can see it.

# Build the things
(cd battery_indicator; cargo build)
(cd animated_indicator; cargo build)
(cd pi_host; cargo build)

# Move the wasm file
cp ./target/wasm32-unknown-unknown/debug/battery_indicator.wasm ./indicator.wasm