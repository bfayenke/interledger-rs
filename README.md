<p align="center">
  <img src="ilprs.svg" width="700" alt="Interledger.rs">
</p>

---
> Interledger implementation in Rust :money_with_wings:

[![crates.io](https://img.shields.io/crates/v/ilp.svg)](https://crates.io/crates/ilp)
[![CircleCI](https://circleci.com/gh/emschwartz/ilp-rs.svg?style=shield)](https://circleci.com/gh/emschwartz/ilp-rs)
[![codecov](https://codecov.io/gh/emschwartz/ilp-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/emschwartz/ilp-rs)

## Installation
- Install Rust (using [`rustup`](https://rustup.rs/) is recommended)
- Install [`moneyd`](https://github.com/interledgerjs/moneyd)
- `cargo install ilp`

## Usage

Make sure you're running `moneyd` first! (`moneyd local` can be used for testing purposes)

### Running an SPSP Server

`ilp spsp server --port 3000`

(You can see the full options by running `ilp spsp server --help`)

### Sending an SPSP Payment

`ilp spsp pay --receiver http://localhost:3000/bob --amount 1000`

(You can see the full options by running `ilp spsp pay --help`)
