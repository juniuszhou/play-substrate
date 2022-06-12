#!/usr/bin/env bash

../para/target/release/parachain-collator --alice --collator --force-authoring \
--base-path tmp/para --port 30335 --ws-port 9945 --rpc-port 9935 \
--execution wasm --chain dev \
-- --execution wasm --chain tmp/rococo-local-chain-spec.json \
--port 30336 --ws-port 9946 --rpc-port 9936 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp