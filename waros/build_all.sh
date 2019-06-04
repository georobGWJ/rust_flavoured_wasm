#!/bin/bash

# Note: Running 'cargo build' from /waros/ should and often does
# successfully build the project completely. On at least one OSX laptop (mine),
# this command tries to link the binaries to the wasm files and fails at this 
# linking stage.

# In any case, the bot wasm files need to be moved to /waros/consolerunner/bots
# before consolerunner will run. This bash script builds everything and moves the
# bot files automatically.

# Build the core logic without linking - this builds the debug versions
echo "Building binaries..."
(cd botengine;     cargo build)
(cd warsdk;        cargo build)
(cd consolerunner; cargo build)

# Build the bot wasm files
# These have .cargo/config files in their directories that tell cargo to
# target 'wasm32-unknown-unknown'
echo "Building wasm bots..."
(cd dumbotrs; cargo build)
(cd rabbit;   cargo build)
(cd rook;     cargo build)

# Finally, move the compiled bot wasm files to an arbitrary directory
# that console runner is hardcoded to look in
echo "Moving bot files..."
wasm_build_folder=./target/wasm32-unknown-unknown/debug
wasm_target_folder=./consolerunner/bots
cp $wasm_build_folder/dumbotrs.wasm  $wasm_target_folder/dumbotrs.wasm
cp $wasm_build_folder/rabbit.wasm    $wasm_target_folder/rabbit.wasm
cp $wasm_build_folder/rook.wasm      $wasm_target_folder/rook.wasm

echo "Done!"