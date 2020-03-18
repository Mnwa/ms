# MS converter library
[![](https://docs.rs/ms-converter/badge.svg)](https://docs.rs/ms-converter/)
[![](https://github.com/Mnwa/ms/workflows/build/badge.svg?branch=master)](https://github.com/Mnwa/ms/actions?query=workflow%3Abuild)
[![](https://img.shields.io/crates/v/ms-converter.svg)](https://crates.io/crates/ms-converter)
[![](https://img.shields.io/crates/d/ms-converter.svg)](https://crates.io/crates/ms-converter)

Fast abstraction for converting human-like times into milliseconds.
Like, are `1d` to `86400000`.

There are two ways to calculate milliseconds:
* In the runtime `crate::ms_converter::ms`
* In the compilation time `crate::ms_converter::ms_expr`

## Getting Started

### Usage
Add this to your Cargo.toml:

```toml
[dependencies]
ms-converter = "0.6"
```

### Examples

#### Running ms converter in Runtime:
```rust
use crate::ms_converter::ms;

let value = ms("1d").unwrap();
assert_eq!(value, 86400000)
```

#### Convert ms in the compilation step:
```rust
use crate::ms_converter::ms_expr;

const VALUE: i64 = ms_expr!(i64, 1 d);
assert_eq!(VALUE, 86400000)
```

#### Convert ms into `time.Duration`
```rust
use crate::ms_converter::ms_into_time;

let value = ms_into_time("1d").unwrap();
assert_eq!(value.as_millis(), 86400000)
```

## Performance
You can check the performance diff between `ms_converter` and `ms` libraries [here](Benchmark.md).

Also, the macro has no time in the runtime! It will be converted into the const value.

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
