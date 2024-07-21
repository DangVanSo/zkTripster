# zkTripster: Time Release Incentive Platform for Security Threats Ethical Reporting

> **TL;DR:** generate proof of smart contract vulnerability and time-lock encrypt the exploit calldata to be released after a time sufficient to patch the issue. In the meantime, a smart contract owner can get that calldata earlier if they transfer ETH to a white hacker’s address.

This idea builds on top of [verifiable time-lock encryption](https://drand.love/docs/timelock-encryption) and [verifiable vulnerability disclosure](https://blog.trailofbits.com/2020/05/21/reinventing-vulnerability-disclosure-using-zero-knowledge-proofs/).

The notion of time-lock encryption (TLE) can be expressed simply: ciphertexts are guaranteed to be decryptable after a specified point in time. One can make it verifiable by running encryption procedure inside an arithmetic circuit.

[zkPoEX](https://github.com/zkoranges/zkPoEX) (zk proof of exploit) is a second key ingredient that lets white-hat hackers to prove that they found a vulnerability without revealing the actual exploit information right away. Its aim is to facilitate communication and trustless collaboration between vulnerability researchers and DeFi application developers.

The coordinated vulnerability disclosure (CVD, aka responsible disclosure) is a model in which a vulnerability is disclosed to the public only after the responsible parties have been allowed sufficient time to patch or remedy the vulnerability.

The aim of this project is to create an end-to-end verifiable cryptographic infrastructure that facilitates coordinated vulnerability disclosure and contingent transactions for trading exploit information for monetary reward. 

Also, see a [full proposal](https://hackmd.io/@timofey/HJmw2StXA) note.

## Problem

Bug bounty programs in the DeFi space can be hard to run and maintain, not always honored, and may not always offer sufficient compensation for white hats. This can lead to a lack of incentive for hackers to report vulnerabilities, which can ultimately result in a less secure DeFi ecosystem. Previous work had laid down basic tools for proving vulnerabilities via ZK proofs. 

Without the conditional payment infrastructure, however, parties must trust an inherently trusted intermediary to exchange vulnerability information for reward which limits their usefulness. 

From Vendors perspective it is often cheaper and easier to pay white hat to remain quite and drag patching the vulnerability as long as possible. All the while, black hats may find and exploit the vulnerability leveraging Vendor's slow response act. Public vulnerability disclosure acts as the only effective way of keeping Vendor's accountable and efficient in patching vulnerabilities.

## Requriements

- [Go](https://go.dev/doc/install)
- [Rust](https://rustup.rs/)
- [SP1](https://succinctlabs.github.io/sp1/getting-started/install.html)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)

## Usage

## Generate zkPoEX

```bash
RUST_LOG=info cargo run --package zkpoex-script --bin prove --release -- --calldata `<calldata>`
```

## Generate Proof of Key Exchange

```bash
RUST_LOG=info cargo run -p ecdh-script --bin prove -r -- --local-sk <LOCAL_SK> --vendor-pk <VENDOR_PK>
```

## Run White Hat Server

```bash
RUST_LOG=info cargo run --package server --bin cli -r -- prover --calldata `<calldata>` -b `<bounty-bid>`
```

## Deploy Platform UI

```bash
cd frontend && yarn dev
```
