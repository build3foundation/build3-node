#!/bin/sh
# Shell script for mac to start test node
# Use the chmod 755 yourscript.sh command to make it executable
# make sure to purge existing chain data first with `build3-node purge-chain
# --base-path /tmp/node01 --chain local -y`
#
# DO NOT USE FOR PRODUCTION!
# rpc-methods is set to Unsafe which is for local development only.

build3-node \
--base-path /tmp/node01 \
--chain ./customSpecRaw.json \
--port 30333 \
--ws-port 9945 \
--rpc-port 9933 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name block_choy_1
