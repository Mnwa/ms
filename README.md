# MS library

Fast converter various time formats into milliseconds.
Like are `1d` to `86400000`

## Getting Started

### Usage
Add this to your Cargo.toml:

```toml
[dependencies]
ms-converter = "0.4"
```

### Example

```rust
use crate::ms_converter::ms;

let value = ms("1d").unwrap();
assert_eq!(value, 86400000_f64)
```

## Performance
You can check performance diff between `ms_converter` and `ms` libraries [here](Benchmark.md).

## Running the tests

### Unit tests

```bash
cargo test
```

### Coding style tests

Running code style tests

```
cargo fmt --all -- --check
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
