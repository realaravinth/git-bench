# Reading file off of Git vs File System benchmark

I'm investigating versioning methods for
[realaravinth/pages](https://github.com/realaravinth/pages)(self-hosted
GitHub pages). Using `Git`, as opposed to having multiple copies of the
same website, will use less storage but will add fs reading overhead as
the Git database will have to be queried first before the contents of
the file can be fetched.

File querying is implemented for the following systems:

1. Git via [`git2-rs`](https://crates.io/crates/git2)
2. Vanilla file system([`std::fs::read`](https://doc.rust-lang.org/std/fs/fn.read.html))
3. Cached Git
4. Cached file system

## How to run

1. Download `rustup`: see [rustup.rs](https://rustup.rs/)

2. Download Rust nightly release:

```bash
rustup toolchain install nightly
```

3. Switch to nightly release:

```bash
rustup override set nightly
```

4. Bootstrap test

```bash
./scripts/init.sh
```

5. Run benchmarks

```
cargo bench
```

## Results

Run on Intel Core i7-9750h, NVMe SSD(read: 17,000MB/s), 32GB DDR4 @2667 MHz

```bash
➜  git-bench git:(master) ✗ cargo bench
   Compiling git-bench v0.1.0 (/home/aravinth/code/batsaene/git-bench)
    Finished bench [optimized] target(s) in 0.54s
     Running unittests (target/release/deps/git_bench-91a556763cda354f)

running 4 tests
test tests::bench_cache_fs  ... bench:       2,341 ns/iter (+/- 81)
test tests::bench_cache_git ... bench:       2,366 ns/iter (+/- 59)
test tests::bench_fs        ... bench:     430,702 ns/iter (+/- 15,572)
test tests::bench_git       ... bench:   6,849,857 ns/iter (+/- 834,185)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured; 0 filtered out; finished in 22.02s
```
