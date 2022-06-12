#!/usr/bin/env bash

/Users/junius/github/parachains/polkadot/target/release/polkadot \
--chain tmp/rococo-local-chain-spec.json --bob --base-path tmp/relay-bob \
--port 30334 --ws-port 9944 --rpc-port 9934 \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp

