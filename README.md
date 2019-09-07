cargo-walk
==========

A cargo plugin to allow running a command for each package root path in a
Rust crate dependency tree.

Examples
--------

 - Search all code related to a crate, including dependencies using ripgrep:
   `cargo walk -- rg -C 10 "find me"`
 - List all crate roots in the dependency tree:
   `cargo walk echo` or `cargo walk -- echo`

Make sure to add `--` between `cargo walk` and the command if it contains `-`
or `--` flags.

Changelog
---------

 - __0.1.0__ First version. The root path of each crate in the crate dependency
   tree will be added as the last argument to the command.

