# `tio`
[![Format](https://github.com/maxwellmattryan/tio/actions/workflows/ci.format.yml/badge.svg?branch=develop&event=push)](https://github.com/maxwellmattryan/tio/actions/workflows/ci.format.yml)
[![Test](https://github.com/maxwellmattryan/tio/actions/workflows/ci.test.yml/badge.svg?branch=develop&event=push)](https://github.com/maxwellmattryan/tio/actions/workflows/ci.test.yml)

## Overview
`tio`, short for "Tangle IO", is a CLI tool that offers fast and easy-to-use interaction with the [IOTA](https://iota.org) Tangle.

## Build
Please refer to the [build documentation](./docs/BUILD.md) for details information on how to build `tio`.

## Usage
Please refer to the [usage documentation](./docs/USAGE.md) for detailed information on how to use `tio`.

## Roadmap
Each list below is ordered in terms of priority (higher position = higher priority).

### Features
- `info` command (queries the node information (`api/v1/info` endpoint) of an IOTA node)
- `monitor` command (watches a given address for updates in the IOTA ledger*)
- `spam` command (repeatedly sends a data message to the IOTA Tangle)

\* It is worth noting that the IOTA ledger and the IOTA Tangle are __not the same thing__.
The Tangle contains all messages regardless of whether or not they are data- or value-based.
The IOTA ledger refers to all value-based messages.

### Enhancements
- Clipboard copying (IDs, messages, transactions, data, etc.)
- Better logging and output formatting (indexation, coloring, etc.)
- Better message formatting on `broadcast` (string-ify JSON data)
- More arguments for commands:
    - Node authentication (username, password, and JWT)
    - Specify activity to watch for with `monitor`
    - Time interval between `spam` messages
- Data encryption / decryption including algorithm selection
- Broadcasting non-string data (file inputs to start, may use hash of file contents instead)
- Batch broadcasting and searching of messages

### Fixes
_None_
