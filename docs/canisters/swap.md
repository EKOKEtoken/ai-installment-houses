# Swap Canister

- [Swap Canister](#swap-canister)
  - [Introduction](#introduction)
  - [API](#api)
    - [list](#list)
    - [unlist](#unlist)
    - [icp\_price](#icp_price)
    - [buy](#buy)

## Introduction

The swap canister provides an interface to buy and sell the `AI Installment Houses` NFTs.

The canister provides a simple api to list an NFT for sale at a given ICP price and the possibility to buy listed tokens.

## API

See [DID file](../../src/swap/swap.did)

### list

List an NFT for sale.

Arguments:

- `token_identifier`: the ID of the token to list for sale
- `icp_price`: the ICP price for the token

Conditions:

- the NFT must have `operator` set to `swap canister`
- the NFT must be owned by `caller`

### unlist

Remove from sale an NFT.

Arguments:

- `token_identifier`: the ID of the token to unlist from sale

Conditions:

- The NFT must be owned by `caller`

### icp_price

Get the ICP price for the given token

Arguments:

- `token_identifier`: the ID of the token to get the price for

Conditions:

- the NFT must be listed for sale
- the NFT must have `operator` set to `swap canister`

### buy

Buy the provided token

Arguments:

- `token_identifier`: the ID of the token to buy

Conditions:

- the NFT must be listed for sale
- the NFT must have `operator` set to `swap canister`
- the `caller` must have given enough allowance (`icrc2_allowance`) to the `swap canister` for the `ICP` token
