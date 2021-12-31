## Setup
If you have not already, begin by installing the [Rust](https://rust-lang.org) programming language:
```bash
# copied from https://rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone this repository in the directory of your choice:
```bash
# HTTPS
git clone https://github.com/maxwellmattryan/tio.git

# SSH
git clone git@github.com:maxwellmattryan/tio.git
```

## Build
Compile the release version of the repo (use `develop` branch for latest):
```bash
# in cloned directory of tio repository
cargo build --release
```

Execute a command with the newly built binary*:
```bash
# display the help information for tio
./target/release/tio help
```

\* You may consider adding the path in which you built the binary to the `PATH` environment variable, making it executable from _any_ location.
