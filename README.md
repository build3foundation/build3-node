# build3-node

The blockchain node for Build3 built on substrate.

This repository is tracking Substrate's `master`.

## Installation

This installation assumes you have `Rust` and `Cargo` installed on your local environment.
Mac:

```bash
sudo cargo install build3-node --path ./node
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
