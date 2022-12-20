# rust-coverage-integration-test

[![Coverage Status](https://coveralls.io/repos/github/pldubouilh/rust-coverage-integration-test/badge.svg?branch=main)](https://coveralls.io/github/pldubouilh/rust-coverage-integration-test?branch=main)

![Untitled Diagram drawio (1)](https://user-images.githubusercontent.com/760637/208670490-c9c8c3be-0940-4b4f-a784-c8dbf3f44e19.png)

end-to-end integration-test tool for rust, that's easy to extend, and supports coverage ([locally with grcov](https://pldubouilh.github.io/rust-coverage-integration-test), or with [hosted services](https://coveralls.io/github/pldubouilh/rust-coverage-integration-test))

needs a rustup component, and grcov
```sh
% rustup component add llvm-tools-preview
% cargo install grcov
```
