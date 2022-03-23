# build3-node

The blockchain node for Build3 built on substrate.

This repository has two forks of Parity repositories included as submodules.
The repositories are `ink` and `substrate` and they are intended to give direct
access to the `master` while being able to update the code if needed to correct
bugs or improve features. If our team improves or corrects the original `ink`
or `substrate` repository, please review the contribution guidelines and offer
the work as a pull request to their work.

Note: [check this out for M1 silicon](https://vikiival.medium.com/run-substrate-on-apple-m1-a2699743fae8)

## Rust Nightly

```shell
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

## Installation

This installation assumes you have `Rust` is installed on your local environment. For windows usersnot using wsl, see section below . 
Mac:

```bash
cargo install build3-node --path ./node
```

## Usage

To run a local dev node execute

```bash
build3-node --dev
```

A new chain in temporary directory will be created each time the command is
executed. This is the default for `--dev` chain specs.

If you want to persist chain state across runs you need to specify a directory with `--base-path`.

### Show only Errors and Contract Debug Output

To have only errors and contract debug output show up on the console you can
supply `-lerror,runtime::contracts=debug` when starting the node.

Important: Debug output is only printed for RPC calls or off-chain tests ‒ not for transactions!

## Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect to it with the **Polkadot-JS Apps**
frontend to interact with your chain.
[Click here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944)
to connect the frontend to your local node.

-[] Add council
-[] Add identity

## NPOS Network Setup

[This](https://medium.com/oak-blockchain/tutorial-deploy-a-substrate-npos-network-in-3-minutes-fab9de5c4c2b) was a useful website to understand how to get the NPOS staging network
setup with custom keys.

## Windows Users
Follow [these](https://docs.substrate.io/v3/getting-started/windows-users/) steps.
