#!/bin/sh
# Shell script for mac to start test node
# Use the chmod 755 yourscript.sh command to make it executable

build3-node \
--base-path /tmp/bob \
#--chain ./customSpecRaw.json \
--bob \
--chain local \
--port 30334 \
--ws-port 9946 \
--rpc-port 9934 \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--validator \
--rpc-methods Unsafe \
--name bob \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWKh3D4QANyj9hADNSqtuV5sToovGjdQBsS4HSsY3FCpCQ
