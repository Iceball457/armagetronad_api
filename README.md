# Usage

1. Clone or download/extract this repository
2. Add the following to your cargo.toml

```toml
[dependencies]
armagetronapi = { path = "/path/to/local/repository" }
```

For example, I usually use `path = "../armagetronad_api"`

# Contributing

There are 1028 total console commands that need to be implemented. 1 command currently has an implementation.

Fork this repo and add implementations to functions wherever you see a `todo!();` then create a pull request.

Please list the functions you implement in the body of your PR! A maintainer will adjust the count in this README.
