# Internals

Mangrove uses a custom packaging system that is designed to meet the Mangrove design principles:

- the inner working of the package manager should be hidden to the average user
- it should be _fast_ - everything should be build for speed
- it should be cross-platform - it should work on \*nix, macOS, and Windows
- it should be free and open source for anyone to audit or modify as needed
- minimum system requirements should be as low as possible

The current implementation accomplishes these things by doing the following:

- providing a [comprehensive command line interface](cli/index.md) to hide implementation details from the user
- it is written in Rust and uses a binary format for as many things as possible
- it will run anywhere Rust will compile, as long as there is a filesystem and an internet connection
- it is open source and licensed under the GNU GPLv3
- filesizes are minimized and memory constraints are a priority

To start, get a high level view of all actions in the package manager: [transactions](internals/transactions.md)
