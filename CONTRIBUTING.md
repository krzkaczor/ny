# Contributing

All contributions are welcomed.

I am by no means a rust expert. This is my very first open-source rust project. Let me know if you have any ideas how to make this code more idiomatic! :)

## Development

This is a pretty standard rust app, use `cargo` for everything.

```
cargo test # we have pretty extensive test suite
cargo build # dev build is needed to run E2E tests
cd test-e2e
bun test
```

## Architecture

- `common/cli` - this is where cli args are parsed. We use a combination of clippy and hand written pre-processing to deal with some edge cases,
- `common/fs` - minimal fs abstraction, used to make writing unit tests possible,
- `common/execute` - minimal spawn abstraction, used to make writing unit tests possible,
- [e2e tests](./test-e2e/README.md)

## Changelog management

We use [changesets](https://github.com/changesets/changesets) to manage changes. When you're done with a **user facing change**, run `ny changeset add` in the root of the project to kick off a CLI for adding a new changeset. Describe your change and commit the resulting file.
