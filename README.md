# CLI URL Parser

Simple URL query parser.

## Installation

With `cargo`:

```sh
cargo install parse_url
```

## Usage

Use:

```sh
parse_url "https://github.com/iamelevich/parse_url?test1=ewrw&test2=edsdsdf"
```

Output will be:

```blank
test1=ewrw
test2=edsdsdf
```
