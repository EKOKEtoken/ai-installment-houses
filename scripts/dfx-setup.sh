#!/bin/bash

dfx stop
dfx start --background --clean
dfx canister create ledger
dfx canister create swap

dfx stop
