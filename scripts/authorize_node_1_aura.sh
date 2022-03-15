#!/bin/sh
# Shell script for mac to authorize the test node.
# Save your secret to local ENV first.
# Use the chmod 755 yourscript.sh command to make it executable

build3-node key insert --base-path /tmp/node01 \
--chain customSpecRaw.json \
--scheme Sr25519 \
--suri "${NODE_1_AURA_SEED}" \
--password-interactive \
--key-type aura
