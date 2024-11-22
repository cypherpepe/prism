<p align="center">
  <picture>
    <source srcset="./assets/prism-white.png" media="(prefers-color-scheme: dark)">
    <img src="./assets/prism-dark.png" alt="Prism" width="350">
  </picture>
</p>

# prism

[![delta devs](https://img.shields.io/badge/building-in_stealth-0097FF)](https://deltadevs.xyz)
![Dependencies](https://img.shields.io/badge/dependencies-up%20to%20date-0097FF.svg)
[![GitHub Issues](https://img.shields.io/github/issues-raw/deltadevsde/transparency-dictionary?color=0097FF)](https://github.com/deltadevsde/transparency-dictionary/issues)
![Contributions welcome](https://img.shields.io/badge/contributions-welcome-0097FF.svg)
[![License](https://img.shields.io/badge/license-MIT-0097FF.svg)](https://opensource.org/licenses/MIT)

**global identity layer enabling automatic verification of end-to-end encrypted services, providing users with trust-minimized security and privacy through transparent key management.**

## What is Prism?

Prism is a decentralized key transparency protocol, first inspired by the paper [Tzialla et. al](https://eprint.iacr.org/2021/1263.pdf), leveraging zkSNARKs to enable trust-minimized verification of E2EE services via WASM light clients. This eliminates the possibility for hidden backdoors in E2EE services through a user-verifiable key management system. It uses transparency dictionaries under the hood, offering a generalized solution for managing a label-value map in environments where the service maintaining the map is not completely trusted.

Prism provides the first key-transparency solution to enable automatic verification of the service provider. This is achieved by providing constant size succinct proofs to WASM light clients over a data availbility layer. The system is designed to be efficient, scalable and secure, making it suitable for a wide range of applications.

You can view further information about the project in our [documentation](https://prism.deltadevs.xyz). The project is undergoing rapid development. You can view the current development status [here](https://prism.deltadevs.xyz/state).


## Status

The project is still in the early development phase, has not been audited, and is not yet suitable for use in production environments.

Due to this ongoing development work, changes are still being made that may affect existing functionalities.

## Circuits
We are currently experimenting with various proof systems and have handwritten groth16 and supernova circuits to handle the epoch proofs. We are also experimenting with SP1 as an alternative, which you can find in the `prism-sp1` crate.

## Installation

### Prerequisites

### Install Dependencies

We use `just` as a task runner. Once installed, you can install the rest of the dependencies with:

```bash
just install-deps
```

### Building

To build the project, run:

```bash
just build
```

This will compile the `prism-cli` binary and sp1 `ELF` that are used to run the prover, light-client, and full-node.

### Running a local DA layer

To run a local Celestia network for testing, use:

```bash
just celestia-up
```

### Starting the prover

If the dependencies are installed and the local devnet is running, a prism node can be started.

Prism can be started in three different ways:
1. as a prover (service provider and proof generator)
2. as a light-client (to verify the proofs posted on Celestia using the cryptographic commitments)
3. as a full-node (acts as a service provider, processing all transactions and making the state available to the light-clients)

To start the prover, run:
```bash
prism-cli prover
```

This will output the prover's verifying key in the logs, which you can use along with the light-client and full-node to verify the proofs.

to start the light-client, run the following command:

```bash
prism-cli light-client|full-node --verifying-key <verifying-key>
```

You can then interact with Prism via the interfaces defined in [webserver.rs](https://github.com/deltadevsde/prism/blob/main/crates/prover/src/webserver.rs).

## Contributions

Contributions are welcome! Please refer to our [contributing guidelines](CONTRIBUTING.md) for information on how to submit pull requests, report issues, and contribute to the codebase.
