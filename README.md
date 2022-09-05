# rust-coverage-integration-test

[![Coverage Status](https://coveralls.io/repos/github/pldubouilh/rust-coverage-integration-test/badge.svg?branch=main)](https://coveralls.io/github/pldubouilh/rust-coverage-integration-test?branch=main)

end-to-end integration test coverage measurement with rust and grcov

needs a rustup component, and grcov
```sh
% rustup component add llvm-tools-preview
% cargo install grcov
```

supports both [coveralls](https://coveralls.io/github/pldubouilh/rust-coverage-integration-test) and local reports ([example](https://pldubouilh.github.io/rust-coverage-integration-test/))
