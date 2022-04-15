# Specifying packages

Packages can be specified in several ways:

| In order to get | You should use |
|---|---|
| The latest avaliable version of the package from any source | `pkgname` |
| The latest avaliable version of the package that is earlier than a specific version | `pkgname<maxver` |
| The latest avaliable version of the package that is earlier or equal to a specific version | `pkgname<=maxver` |
| The package at a specific version | `pkgname@version` |
| The latest avaliable version of the package that is later or equal to a specific version | `pkgname>=minver` |
| The latest avaliable version of the package that is later than a specific version | `pkgname>minver` |

## Regex

All pkgspecs will meet the following (long) regex:

```regex
^([a-zA-Z][a-zA-Z\-_]+\/)?([a-zA-Z][a-zA-Z0-9\-_]+)((<|<=|@|>=|>)((0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?))?$
```
