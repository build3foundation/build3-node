#!/bin/sh
./target/release/node-template benchmark \
    --chain dev \               # Configurable Chain Spec
    --execution wasm \          # Always test with Wasm
    --wasm-execution compiled \ # Always used `wasm-time`
    --pallet pallet_example \   # Select the pallet
    --extrinsic '*' \          # Select the benchmark case name, using '*' for all
    --steps 20 \                # Number of steps across component ranges
    --repeat 10 \               # Number of times we repeat a benchmark
    --json-file=raw.json \      # Optionally output json benchmark data to a file
    --output ./                 # Output results into a Rust file
