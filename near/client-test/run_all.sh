#!/bin/bash

## show the commands
set -v

## contract accountId: whitelist1.testnet
## secure passphrase:  junk grocery essay such page payment wine fury broccoli forest pet topic

## show state of whitelist1.testnet
near state whitelist1.testnet

## delete sub account, make sure to register whitelist1.testnet first, this also fails if the sub account hasn't been created yet and it's ok if it fails.
near delete app1.whitelist1.testnet whitelist1.testnet

## create sub account under our master account
near create-account app1.whitelist1.testnet --masterAccount whitelist1.testnet

## show state of sub account
near state app1.whitelist1.testnet

## deploy the contract to the sub account
near deploy --wasmFile out/main.wasm --accountId app1.whitelist1.testnet

