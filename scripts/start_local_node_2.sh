#!/bin/sh
# Shell script for mac to start test node
# Use the chmod 755 yourscript.sh command to make it executable

build3-node \
--base-path /tmp/node02 \
--chain ./customSpecRaw.json \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name block_choy_2 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWJFnvgMHjLy7kKNPhxzAU5MrX6dV76kt8d5WuBw5zdRLS
