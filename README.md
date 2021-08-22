# `tio`

## Overview

`tio`, short for "Tangle IO", is a CLI tool written in Rust that offers basic messaging broadcasting (and searching) via the [_IOTA_](https://iota.org) protocol. 

## Features

- broadcast message on the IOTA Tangle
- search for message on the IOTA Tangle
- encrypt and decrypt message payloads

## Usage

- Broadcasting messages:
```bash
USAGE:
    tio broadcast [OPTIONS] [message]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --network <network>    Particular IOTA network to broadcast to ("mainnet", "devnet", and "testnet")

ARGS:
    <message>    Data to embed in message payload (must be < 4kb and UTF-8 encoded)
```

## Planning

I have never written a _actual_ Rust tool, program, i.e. anything. 
This is going to be a fun project learning how to write functional and practical Rust using the IOTA protocol, which I have an interest in.

### Design

`tio` should be fast (no reason _not_ to be) with minimal overhead.  
I want to focus mainly on the interesting parts, i.e. interacting with the IOTA protocol (using a client library).
It should use other open-source libraries where possible (perhaps use [`crypto.rs`](https://github.com/iotaledger/crypto.rs) from [iotaledger](https://github.com/iotaledger)).

### Nice-to-Have Features

The following features are not necessary for a MVP but would be nice to have as they allow for more interesting things you can (potentially) do with `tio`.

- add configuration for specific node URLs
- add support for data types besides strings (basically add file IO, but basic stuff at first - `.txt`, `.json`, etc.)
- add support to save message result to file
- add nicely formatted console output (and colored)
- add `spam` command (this will be good for multi-threading practice)
- add support for batching broadcasting directory of files
- add different encryption types ("serious" users will most likely encrypt the data before and after using this tool simply as _just_ a communication tool)
