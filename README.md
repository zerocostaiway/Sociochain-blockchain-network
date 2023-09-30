
# SocioChain Blockchain
<a href="https://ibb.co/CKZg2Nh"><img src="https://i.ibb.co/hFjbfhD/Socio-Chain-logo-color.png" alt="Socio-Chain-logo-color" border="0"></a>

[![StackExchange](https://img.shields.io/badge/StackExchange-Community%20&%20Support-222222?logo=stackexchange)](https://substrate.stackexchange.com/)

Welcome to the SocioChain blockchain – a groundbreaking platform designed to revolutionize the digital economy by making social capital the heart of our ecosystem. SocioChain empowers users to monetize their contributions and participate in a dynamic value exchange network.

## [Polkadot](./polkadot/)
[![PolkadotForum](https://img.shields.io/badge/Polkadot_Forum-e6007a?logo=polkadot)](https://forum.polkadot.network/)
[![Polkadot-license](https://img.shields.io/badge/License-GPL3-blue)](./polkadot/LICENSE)

Implementation of a node for the https://polkadot.network in Rust, using the Substrate framework. This directory
currently contains runtimes for the Polkadot, Kusama, Westend, and Rococo networks. In the future, these will be
relocated to the [`runtimes`](https://github.com/polkadot-fellows/runtimes/) repository.

## [Substrate](./substrate/)
 [![SubstrateRustDocs](https://img.shields.io/badge/Rust_Docs-Substrate-24CC85?logo=rust)](https://paritytech.github.io/substrate/master/substrate/index.html)
 [![Substrate-license](https://img.shields.io/badge/License-GPL3%2FApache2.0-blue)](./substrate/README.md#LICENSE)

Substrate is the primary blockchain SDK used by developers to create the parachains that make up the Polkadot network.
Additionally, it allows for the development of self-sovereign blockchains that operate completely independently of
Polkadot.

## [Cumulus](./cumulus/)
[![CumulusRustDocs](https://img.shields.io/badge/Rust_Docs-Cumulus-222222?logo=rust)](https://paritytech.github.io/cumulus/cumulus_client_collator/index.html)
[![Cumulus-license](https://img.shields.io/badge/License-GPL3-blue)](./cumulus/LICENSE)

Cumulus is a set of tools for writing Substrate-based Polkadot parachains.

## Upstream Dependencies

Below are the primary upstream dependencies utilized in this project:

- [`parity-scale-codec`](https://crates.io/crates/parity-scale-codec)
- [`parity-db`](https://crates.io/crates/parity-db)
- [`parity-common`](https://github.com/paritytech/parity-common)
- [`trie`](https://github.com/paritytech/trie)

## Security

The security policy and procedures can be found in [docs/SECURITY.md](./docs/SECURITY.md).

## Contributing & Code of Conduct

Ensure you follow our [contribution guidelines](./docs/CONTRIBUTING.md). In every interaction and contribution, this
project adheres to the [Contributor Covenant Code of Conduct](./docs/CODE_OF_CONDUCT.md).

## Additional Resources

- For monitoring upcoming changes and current proposals related to the technical implementation of the Polkadot network,
  visit the [`Requests for Comment (RFC)`](https://github.com/polkadot-fellows/RFCs) repository. While it's maintained
  by the Polkadot Fellowship, the RFC process welcomes contributions from everyone.
