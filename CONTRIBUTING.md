# Contributing

All contributions are welcomed.

I am by no means a rust expert. This is my very first open-source rust project. Let me know if you have any ideas how to make this code more idiomatic! :)

## Development

This is a pretty standard rust app, use `cargo` for everything.

```
cargo test # we have pretty extensive test suite
```

## Architecture

- `common/cli` - this is where cli args are parsed. We use a combination of clippy and hand written pre-processing to deal with some edge cases,
- `common/fs` - minimal fs abstraction, used to make writing unit tests possible,
- `common/execute` - minimal spawn abstraction, used to make writing unit tests possible,