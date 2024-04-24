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
- `dip721_name`
- `dip721_symbol`
- `dip721_logo`
- `dip721_total_unique_holders`
- `dip721_token_metadata`

    ```json
    {
      "method": "dip721_token_metadata",
      "params": {
        "id": 1
      }
    }
    ```

- `dip721_total_supply`

## Token Metadata

Each NFT has the following properties, following the DIP721 standard.

- ...
