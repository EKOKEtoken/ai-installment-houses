# AI Installment Houses

![CI state](https://github.com/EKOKEtoken/ai-installment-houses/workflows/build-test/badge.svg)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

Powered by **Internet Computer**

---

- [AI Installment Houses](#ai-installment-houses)
  - [Introduction](#introduction)
  - [Documentation](#documentation)
  - [Whitepaper](#whitepaper)
  - [Canisters](#canisters)
  - [Get started](#get-started)
    - [Dependencies](#dependencies)
    - [Build canisters](#build-canisters)
  - [License](#license)

---

## Introduction

...

## Documentation

Read the [Project Documentation](./docs/README.md)

## Whitepaper

You can find the whitepaper on our website <https://www.ekoketoken.com/whitepaper>

## Canisters

- **ledger**: `...`
- **swap**: `...`

## Get started

### Dependencies

Before getting started with ekoke, you need to install these dependencies:

- Rust >= 1.74

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

- Dfx >= 0.16

    ```sh
    sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)
    dfx extension install sns
    ```

- cargo-make

    ```sh
    cargo install cargo-make
    ```

- Wasm32 target

    ```sh
    rustup target add wasm32-unknown-unknown
    ```

### Build canisters

In order to build canister you need to setup the dfx environment and then build the source code, luckily all these steps are automated with cargo-make.

```sh
cargo make dfx-setup
cargo make dfx-build
```

## License

You can read the entire license [HERE](LICENSE)
