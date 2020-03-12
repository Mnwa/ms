# MS library

Fast converter various time formats into milliseconds.
Like are `1d` to `86400000`

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

For use this lib you need Rust and Cargo on local machine

```
Give examples
```

### Usage
Add this to your Cargo.toml:

```toml
[dependencies]
ms-converter = "0.3"
```

### Example

```rust
use crate::ms_converter::ms;

let value = ms("1d").unwrap();
assert_eq!(value, 86400000_f64)
```

## Running the tests

### Unit tests

```bash
cargo +nightly test
```

### Coding style tests

Running code style tests

```
cargo +nightly fmt --all -- --check
```

## Contributing

Just create pr or issue. You welcome.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/Mnwa/ms/tags). 

## Authors

* **Mikhail Panfilov** - *Initial work* - [Mnwa](https://github.com/Mnwa)

See also the list of [contributors](https://github.com/Mnwa/ms/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details
