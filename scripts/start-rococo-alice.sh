#!/usr/bin/env bash

/Users/junius/github/parachains/polkadot/target/release/polkadot \
--chain tmp/rococo-local-chain-spec.json --alice --base-path tmp/relay-alice \
--port 30333 --ws-port 9943 --rpc-port 9933 \
--node-key 0000000000000000000000000000000000000000000000000000000000000001

