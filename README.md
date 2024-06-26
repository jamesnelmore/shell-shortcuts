This CLI utility is designed to easily manage shell aliases 
by providing a way to add and remove aliases from anywhere.

# Installation
1. Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).
2. Clone the repo with `https://github.com/jamesnelmore/shell-shortcuts.git`.
3. Run `./deploy.sh` to build the app and copy it to `/usr/local/bin` as the binary `sshort`.
    - Alternatively, build the app with `cargo build --release` 
    and then copy the binary in `target/release` to somewhere in your PATH.

# Use
This app will modify the aliases present in the `~/.aliases` file, 
which should be sourced from the shell profile.
This may be extended. in #7.

For available commands, run `sshort help`.
