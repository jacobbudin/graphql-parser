# GraphQL Parser

A [GraphQL](http://graphql.org) query parser written in [Rust](https://www.rust-lang.org) using [Pest](https://github.com/pest-parser/pest).

## Usage

This parser has a single public function `parse` that accepts a `str` reference and returns a vector of Pest tokens.

```rust
parse("query { ... }");
```

## License

[3-Clause BSD License](LICENSE)
