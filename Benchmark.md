# Benchmarks
There is diff between `ms` library and `ms-converter`

## Perfomance diff
```text
running 2 tests
test tests::bench_ms_converter ... bench:          27 ns/iter (+/- 2)
test tests::bench_ms_second    ... bench:     302,158 ns/iter (+/- 62,937)
```

`ms_converter` is faster than `ms` by more than 11191 times!

## Environment
* OS `macOs catalina 10.15.3`
* Rust `1.41.0`

## Metal
* `MacBook Pro (16-inch, 2019)`
* Processor `2,3 GHz 8-Core Intel Core i9`
* Memory `16 GB 2667 MHz DDR4`

## Test command
```bash
cargo +nightly bench -- --exact
```

## Code

```rust
#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use ms_converter::ms as ms_converter;
    use ms::*;
    use test::Bencher;

    #[bench]
    fn bench_ms_converter(b: &mut Bencher) {
        b.iter(|| ms_converter("1d").unwrap());
    }
    #[bench]
    fn bench_ms_second(b: &mut Bencher) {
        b.iter(|| ms!("1d").unwrap());
    }
}
```