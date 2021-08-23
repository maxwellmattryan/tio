# `tio`

## Overview

`tio`, short for "Tangle IO", is a CLI tool written in Rust that offers basic messaging broadcasting and searching via the [_IOTA_](https://iota.org) protocol.

## Usage

- Broadcasting a message to the Tangle:
```bash
USAGE:
    tio broadcast [OPTIONS] [data]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --network <network>    IOTA Tangle network to use ("mainnet", "devnet", and "testnet") ("mainnet", "devnet", and "testnet")

ARGS:
    [data]    Data to use as the message payload (must be < 4kb and UTF-8 encoded)
```

- Searching for message on the Tangle:
```bash
USAGE:
    tio search [OPTIONS] <hash>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --network <network>    IOTA Tangle network to use ("mainnet", "devnet", and "testnet") ("mainnet", "devnet", and "testnet")

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

### Planned Features

The following features are not necessary for a MVP but are important features that I plan on integrating into `tio`.

- copying from / pasting to clipboard
- abiliy to encrypt / decrypt messages
- configuration for specific node URLs
- support for `Transaction` payloads
- ability to save message result to file
- add nicely formatted console output (+ colors)

### Nice-to-Have Features

The following features are unnecessary, but would allow for more interesting things to do with `tio`.

- add `spam` command (this will be good for multi-threading practice)
- add support for data types besides strings (basically add file IO, but basic stuff at first - `.txt`, `.json`, etc.)
- add support for batching broadcasting directory of files
- add different encryption types ("serious" users will most likely encrypt the data before and after using this tool simply as _just_ a communication tool)
