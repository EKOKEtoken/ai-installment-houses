#!/bin/bash

yarn
yarn build

# build rust canisters
dfx build
