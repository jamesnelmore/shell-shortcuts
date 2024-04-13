#!/bin/bash

cargo build --release

cp target/release/shell-shortcuts ~/scripts/sshort
echo "Copied to ~/scripts/sshort"
