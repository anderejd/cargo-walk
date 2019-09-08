cargo-walk
==========

[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

A cargo plugin to allow running a command for each package root path in a
Rust crate dependency tree.

Examples
--------

 - Search all code related to a crate, including dependencies using ripgrep:
   `cargo walk -- rg -C 10 "find me"`
 - List all crate roots in the dependency tree:
   `cargo walk echo` or `cargo walk -- echo`
 - List dependencies based on size:
   `cargo walk -- du -d 0 -h | sort -h`

Make sure to add `--` between `cargo walk` and the command if it contains `-`
or `--` flags.

Installation
------------

`cargo install cargo-walk`

Changelog
---------

 - __0.1.0__ First version. The root path of each crate in the crate dependency
   tree will be added as the last argument to the command.

