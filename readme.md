<!-- TODO: Add badges here -->
# Websocket Communication Framework for RipTide
# Overview
During the development of the RipTide filesharing application, it was quickly discovered that a reliable means of communication between the central api and the server agent was required. This is the niche that this crate fufills. It acts as a wrapper over a variety of rust websocket communication protocols from different crates, and allows standardised sending of messages between these different frameworks.

Messages are sent as binary, which helps to cutdown on the overhead of converting to a json (or other) standard first.The downside of this is that serde is required as a dependency. The crate is relatively simple, with a basic testing script provided.

Unfortunately, the server and client crate features are not compatible, so testing this crate is difficult. More tests are required for this to be considered a well-tested crate.

# Development

```sh
    # Install Rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # Run tests
    chmod +x test.sh
    ./test.sh
```

# Contributing

## Code Guidelines
**Please write tests** if we have good test coverage we can avoid any bugs down the line.

Outside of this we use standard Rust formatting for code. This will be enforced through use of [clippy](https://github.com/rust-lang/rust-clippy) and [rustfmt](https://github.com/rust-lang/rustfmt).

## Commit Guidelines
In all commits, please try to follow the [convention for commits](https://www.conventionalcommits.org/en/v1.0.0/#specification).

Ideally aim to push every commit you make, rather than accumulating a large number of commits before pushing, this helps to keep everyone on the same
codebase when collaborating.

The exception for this is that you should not commit non-compiling code to the main branch. Open a new branch and
commit to that instead.

## Use of Pull Requests
Outside of exceptional cases, please always push commits to a new branch and then generate a pull request with your new feature. Automated actions will attempt to validate that your code does not break anything existing, but this doesn't guarantee your code works. Please write tests!