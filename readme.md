# Websocket Communication Framework for RipTide

![Test](https://github.com/riptide-org/ws-com-framework/actions/workflows/test.yml/badge.svg)
[![codecov](https://codecov.io/gh/riptide-org/ws-com-framework/branch/main/graph/badge.svg?token=ALQI2M77DH)](https://codecov.io/gh/riptide-org/ws-com-framework)

## Overview

During the development of the RipTide filesharing application, a reliable means of communication between the central api and the server agent was required. Ws-com-framework is a simple framework
for converting message to/from binary to be sent over websocket implementations.

It relies on protobuf3 and the prost crate internally for these conversions.

## Development

```sh
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install protoc
sudo dnf install protobuf protobuf-devel #F36

# Run tests
cargo test
```

## Contributing

### Code Guidelines

**Please write tests** if we have good test coverage we can avoid any bugs down the line.

Outside of this we use standard Rust formatting for code. This will be enforced through use of [clippy](https://github.com/rust-lang/rust-clippy) and [rustfmt](https://github.com/rust-lang/rustfmt).

### Commit Guidelines

In all commits, please try to follow the [convention for commits](https://www.conventionalcommits.org/en/v1.0.0/#specification).

Ideally aim to push every commit you make, rather than accumulating a large number of commits before pushing, this helps to keep everyone on the same
codebase when collaborating.

The exception for this is that you should not commit non-compiling code to the main branch. Open a new branch and
commit to that instead.

### Use of Pull Requests

Outside of exceptional cases, please always push commits to a new branch and then generate a pull request with your new feature. Automated actions will attempt to validate that your code does not
break anything existing, but this doesn't guarantee your code works. Please write tests!
