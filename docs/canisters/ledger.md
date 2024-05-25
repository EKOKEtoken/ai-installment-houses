# Ledger Canister

- [Ledger Canister](#ledger-canister)
  - [Introduction](#introduction)
  - [API](#api)
    - [HTTP Methods](#http-methods)
      - [Methods](#methods)
  - [Token Metadata](#token-metadata)

## Introduction

The ledger is a canister which provides a **Non-fungible Token (NFT)** which implements the **DIP-721** Standard <https://github.com/Psychedelic/DIP721/blob/develop/spec.md> with the `dip721_` namespace.

The NFT provided is called `AI Installment Houses` and has symbol `AIHOUSE`.

## API

See [DID file](../../src/ledger/ledger.did)

### HTTP Methods

This list contains the http available methods with its parameters and response.

```json
{
  "method": "methodName",
  "params": {...}
}
```

#### Methods

- `dip721_metadata`

    ```json
    {
      "created_at": 1,
      "custodians": [
        ""
      ],
      "logo": "",
      "name": "",
      "symbol": "",
      "upgraded_at": 0,
    }
    ```

- `dip721_name`

    ```json
    { "name": "EKOKE" }
    ```

- `dip721_symbol`

    ```json
    { "symbol": "EKOKE" }
    ```

- `dip721_logo`

    ```json
    { "logo": "base64" }
    ```

- `dip721_total_unique_holders`

    ```json
    {
      "totalUniqueHolders": []
    }
    ```

- `dip721_token_metadata`

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
      "approved_at": 1716650205446390569,
      "approved_by": "be2us-64aaa-aaaaa-qaabq-cai",
      "burned_at": null,
      "burned_by": null,
      "is_burned": false,
      "minted_at": 1716650205446419448,
      "minted_by": "be2us-64aaa-aaaaa-qaabq-cai",
      "operator": "bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe",
      "owner": "be2us-64aaa-aaaaa-qaabq-cai",
      "properties": [
        ["floor", { "TextContent": "12" }],
        ["rooms", { "TextContent": "3" }],
        [
          "gallery",
          {
            "NestedContent": [
              ["1", { "TextContent": "img" }],
              ["2", { "TextContent": "img2" }]
            ]
          }
        ]
      ],
      "token_identifier": [65536],
      "transferred_at": 1716650205446441178,
      "transferred_by": "be2us-64aaa-aaaaa-qaabq-cai"
    }
    ```

- `dip721_total_supply`

    ```json
    {
      "totalSupply": 1200
    }
    ```

- `search`

    In addition to the previous method, there is an additional search method, which can be used to search tokens by parameters.

    Params:

    `limit`: limit number of tokens to return
    `offset`: offset to start with
    `properties`:
      `key` => `%value%`: key to search in properties with vlue contained in it
    `sortBy`: `id` or `mintedAt`

    ```json
    [
      {
        "token_identifier": 1,
        "minted_at": 1,
        "minted_by": "rwlgt-iiaaa-aaaaa-aaaaa-cai",
        "owner": ["rwlgt-iiaaa-aaaaa-aaaaa-cai"],
        "operator": [],
        "transferred_at": [],
        "transferred_by": [],
        "approved_at": [],
        "approved_by": [],
        "is_burned": false,
        "burned_at": [],
        "burned_by": [],
        "properties": [
          ["country", { "TextContent": "IT" }],
          ["city", { "TextContent": "Udine" }],
          ["address", { "TextContent": "Via Antonio Marangoni" }],
          ["civic", { "TextContent": "33" }],
          ["zipCode", { "TextContent": "33100" }],
        ],
      }
    ]
    ```

## Token Metadata

Each NFT has the following properties, following the DIP721 standard.

- title: description title for house
- country: ISO3166 encoded country name
- city
- address
- civic
- zipCode
- floor
- totalFloors
- squareMeters
- rooms
- bathrooms
- price
- thumbnail: base64 encoded
