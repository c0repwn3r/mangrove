% mgve(8) | Mangrove Administrator's Manual

NAME
====
mgve - Default command-line interface for libmangrove

SYNOPSIS
========

`mgve [-h] [-v] [-V] subcommand [<options>]`


DESCRIPTION
===========

mgve (mangrove-cli) is a command-line interface (cli) for libmangrove, which is the reference Mangrove implementation.
This serves as the default and reference CLI implementation.

OPTIONS
=======

`-h, --help`
:   Show the help message and exit

`-v, --version`
:   Show a brief listing of the mangrove-cli and libmangrove versions

`-V, --detailed_version`
:   Show a verbose listing of the libmangrove compilation environment, for debugging

COMMANDS
========

Unless the `-h, --help`, `-v, --version` or `-V, --detailed_version` options are present, one of the below commands must be present:

`create new [name]`
:   Creates a new Mangrove source package in the specified directory. If `[name]` is not provided, it defaults to the current directory.

`create build`
:   Build the Mangrove source package in the current directory.

`inspect [-k <pubkey>] [-l] <package>`
:   Provides a dump of the `<package>`s contents. If `-k` is provided, it will use that key to decrypt the package if it is encrypted. Otherwise, it will try the trustcache. `-l` can be used to use a local trustcache for testing.

BUGS
====
Bugs can be reported and filed at https://git.coredoes.dev/c0repwn3r/mangrove/issues.
If you are using a modified version of `mangrove-cli` or `libmangrove`, please be sure to note this in your report.

For critical security vulnerabilities, please contact a project maintainer directly for more information on responsible disclosure.