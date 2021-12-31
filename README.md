# `tio`
[![Format](https://github.com/maxwellmattryan/tio/actions/workflows/ci.format.yml/badge.svg?branch=develop&event=push)](https://github.com/maxwellmattryan/tio/actions/workflows/ci.format.yml)
[![Test](https://github.com/maxwellmattryan/tio/actions/workflows/ci.test.yml/badge.svg?branch=develop&event=push)](https://github.com/maxwellmattryan/tio/actions/workflows/ci.test.yml)

## Overview
`tio`, short for "Tangle IO", is a CLI tool written in Rust that offers basic messaging broadcasting and searching via the [_IOTA_](https://iota.org) protocol.

## Usage
- Broadcasting a message to the Tangle:
```bash
USAGE:
    tio broadcast [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --network <network>    IOTA Tangle network to use ("mainnet" and "devnet")

ARGS:
    <data>          Data to use as the message payload (must be < 4kb and UTF-8 encoded)
    <data-index>    Data index to use for key indexation
```

- Searching for message on the Tangle:
```bash
USAGE:
    tio search [OPTIONS] <hash>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --network <network>    IOTA Tangle network to use ("mainnet" and "devnet")

ARGS:
    <hash>    Hash of a message (must be hexadecimal string of exactly 32 bytes)
```

## Planning
I have never written a _actual_ Rust tool, program, i.e. anything. 
This is going to be a fun project learning how to write functional and practical Rust using the IOTA protocol, which I have an interest in.

### Design
`tio` should be fast (no reason _not_ to be) with minimal overhead.  
I want to focus mainly on the interesting parts, i.e. interacting with the IOTA protocol (using a client library).
It should use other open-source libraries where possible (perhaps use [`crypto.rs`](https://github.com/iotaledger/crypto.rs) from [iotaledger](https://github.com/iotaledger)).

### Roadmap
- `spam` command: repeatedly sends a data message to the IOTA Tangle
- `monitor` command: watches a given address for updates in the IOTA ledger
- Copying (message and transaction IDs)

### Extra Features
- Nicely formatted console output (+ colors)
- Configuration for specific node URLs
- Ability to encrypt / decrypt messages (+ different encryption algorithms)
- Non-string data types (basically add file IO, but basic stuff at first - `.txt`, `.json`, etc.)
- Batch broadcasting and searching of messages
