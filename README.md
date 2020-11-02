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
    bro [FLAGS] [query]

FLAGS:
    -h, --help         
            Prints help information

        --no-color     
            Disable colored output

        --no-paging    
            Disable piping of the output through a pager

    -s, --search       
            Search if provided query exist in the database
            This searches for entries in the http://bropages.org database
    -V, --version      
            Prints version information


ARGS:
    <query>    
            Command to lookup
```

### Note

- This is an unofficial rust port of [ruby bropages](https://rubygems.org/gems/bropages).
- I don't have any intentions to add the `add`, `no`/`...no` or `thanks` commands.

### TODO
 - [ ] Cache results ?
