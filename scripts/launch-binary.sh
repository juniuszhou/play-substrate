#!/usr/bin/env bash

# remove all old stuff
rm -rf tmp/*

# binary path
POLKADOT_BIN="/Users/junius/github/parachains/polkadot/target/release/polkadot"
PARACHAIN_BIN="../para/target/release/parachain-collator"

# export polkadot spec
$POLKADOT_BIN build-spec --chain rococo-local --disable-default-bootnode --raw > tmp/rococo-local-chain-spec.json

# export para state and wasm
$PARACHAIN_BIN export-genesis-state --chain dev > tmp/genesis-state
$PARACHAIN_BIN export-genesis-wasm --chain dev > tmp/genesis-wasm

# 
