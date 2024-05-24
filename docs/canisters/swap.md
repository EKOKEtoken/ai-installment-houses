# Swap Canister

- [Swap Canister](#swap-canister)
  - [Introduction](#introduction)
  - [API](#api)
    - [HTTP Methods](#http-methods)
      - [Methods](#methods)
    - [list](#list)
    - [unlist](#unlist)
    - [get\_listing](#get_listing)
    - [buy](#buy)

## Introduction

The swap canister provides an interface to buy and sell the `AI Installment Houses` NFTs.

The canister provides a simple api to list an NFT for sale at a given ICP price and the possibility to buy listed tokens.

## API

See [DID file](../../src/swap/swap.did)

### HTTP Methods

This list contains the http available methods with its parameters and response.

```json
{
  "method": "methodName",
  "params": {...}
}
```

#### Methods

- `get_listing`

    ```json
    {
      "method": "dip721_token_metadata",
      "params": {
        "id": 1
      }
    }
    ```

    ```json
    {
      "listing": {
        "seller": {
          "owner": "principal",
          "subaccount": []
        },
        "icp_price": 100,
        "expiration_ns": 10000000000
      }
    }
    ```

    or `{ "listing": null }`

- `get_listings`

    Request:

    ```json
    {
      "offset": 0,
      "limit": 20
    }
    ```

    Response:

    ```json
    {
      "listings": [
        {
          "seller": {
            "owner": "principal",
            "subaccount": []
          },
          "icp_price": 100,
          "expiration_ns": 10000000000
        }
      ],
      "total": 1024
    }
    ```

### list

List an NFT for sale.

Arguments:

- `token_identifier`: the ID of the token to list for sale
- `icp_price`: the ICP price for the token
- `expiration`: timestamp of the end of listing (nanoseconds)

Conditions:

- the NFT must have `operator` set to `swap canister`
- the NFT must be owned by `caller`

### unlist

Remove from sale an NFT.

Arguments:

- `token_identifier`: the ID of the token to unlist from sale

Conditions:

- The NFT must be owned by `caller`

### get_listing

Get the ICP price and the expiration for the given token

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
