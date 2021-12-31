## Broadcast
You can send a data-based message to the IOTA Tangle, specifying both the data and index to be embedded.

You may also specify the particular network in which you broadcast the message to.

### Usage
```bash
USAGE:
    tio broadcast [OPTIONS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --network <network>    IOTA Tangle network to use ("mainnet" and "devnet")

ARGS:
    <index>    Indexation key used in the IOTA Tangle
    <data>     UTF-8 encoded data embedded inside the indexation payload
```

## Info
You can query the node information of any IOTA node given valid client options.

You may also specify the particular network in which you query for info on.

### Usage
_Feature not yet implemented._

## Monitor
You can watch the activity of a specific address (Bech32-encoded __only__, support for other identifiers is planned).

You may also specify the particular network in which you monitor the address on.
If you are watching a devnet address (the address's human-readable part or _HRP_ will be `atoi` rather than `iota`) and are unable to find it, it is likely that the network has been reset since the address had any activity.

### Usage
_Feature not yet implemented._

## Search
You can lookup the contents of a message, specifying its particular message ID.

You may also specify the particular network in which you search for the message on.
If you are looking for a message that was broadcasted on the devnet and are unable to find it, it is likely that the network has been reset since that message was originally broadcasted.

### Usage
```bash
USAGE:
    tio search [OPTIONS] <id>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --network <network>    IOTA Tangle network to use ("mainnet" and "devnet")

ARGS:
    <id>    Hash of a message (must be hexadecimal string of exactly 32 bytes)
```

## Spam
You can repeatedly broadcast messages to the IOTA Tangle in rapid succession.

You may also specify the particular network in which you spam messages to.

### Usage
_Feature not yet implemented._
