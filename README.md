# `tio`

## Overview

`tio`, short for _Tangle IO_, is a CLI tool written in Rust that offers basic messaging broadcasting (and searching) via the [_IOTA_](https://iota.org) protocol. 

## Features

- send message given a string
- read data given a message hash 
- encrypt and decrypt data for messages

## Usage

Sending messages:
```bash
tio send <message>

## e.g.
tio send "Hello, Tangle!"
```

Reading messages:
```bash
tio read <hash_id>

## e.g.
tio read 707096c20143bd35424a8f2ed0b24194b4bfb2614053767c1a9e9a5a0e774dbc
```

Using encryption and decryption:
```bash
## encrypt and send message
tio send [-e|--encrypt] <message>

## decrypt and read message
tio read [-d|--decrypt] <hash_id>
```

## Planning

I have never written a _actual_ Rust tool, program, i.e. anything (lol). 
This is going to be a fun project learning how to write functional and practical Rust using the IOTA protocol, which I have an interest in.

### Design

`tio` should be fast (no reason _not_ to be) with minimal overhead.  
I want to focus mainly on the interesting parts, i.e. interacting with the IOTA protocol (using a client library).
It should use other open-source libraries where possible (perhaps use [`crypto.rs`](https://github.com/iotaledger/crypto.rs) from [iotaledger](https://github.com/iotaledger)).

### Nice-to-Have Features

The following features are not necessary for a MVP but would be nice to have as they allow for more interesting things you can (potentially) do with `tio`.

- add nicely formatted console output (and colored)
- add configuration for specific network (`mainnet` vs `devnet`)
- add configuration for specific node URLs
- add support for data types besides strings (basically add file IO, but basic stuff at first - `.txt`, `.json`, etc.)
- add support to save message result to file
- add `spam` command (this will be good for multi-threading practice)
- add support for batching broadcasting directory of files
- add different encryption types ("serious" users will most likely encrypt the data before and after using this tool simply as _just_ a communication tool)
