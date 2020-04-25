bropages (bro)
------------
A CLI to interact with [bropages.org](http://bropages.org).

Highly readable supplement to man pages. Shows simple, concise examples for commands with syntax highlighting.

[![Crates.io](https://img.shields.io/crates/v/bropages)](https://crates.io/crates/bropages)
![License](https://img.shields.io/crates/l/bropages)

### Screencast

[![A screenshot](./screencast.gif)](./screencast.gif)

### Installation

The binary name for bropages is `bro`.

Currently `bropages` is only available through `cargo`.
```
cargo install bropages
```

### Available options
```
USAGE:
    bro [OPTIONS] [query]

OPTIONS:
    -h, --help
            Prints help information

        --no-color
            Disable syntax highlighting

    -l, --lookup
            Lookup an entry, bro, or just call bro
            This looks up entries in the http://bropages.org database.
    -V, --version
            Prints version information


ARGS:
    <query>
            Command to lookup
```

### TODO:
 - [ ] Cache results ?
