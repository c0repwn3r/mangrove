Transactions
------------

The highest level action in Mangrove is known as a Transaction. A transaction is a collection of other actions, such as installing, updating, or removing packages.

.. note::
    A transaction cannot contain another transaction, to prevent transactions containing themselves

A transaction is **non-persistent**, meaning that it is stored in-memory at runtime.
Transactions can contain the following actions:

- `install` - install a package from a repository
- `update` - a superset of `install`, used to update an already installed package
- `remove` - remove a currently installed package
- `reinstall` - remove, then install a package from a repository

There can be an unlimited number of actions within one transaction, with a few exceptions:

- Mangrove must update itself in a seperate transaction (a transaction containing only a single `install` action)
- All operations are mutually exclusive - i.e. you cannot `install` and `update` the same package in one transaction
- A package cannot be operated on if it has been locked.