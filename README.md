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

## Benchmarks

| Benchmark name                     | File access method  | Cached | Number of files queried/iteration |
| ---------------------------------- | ------------------- | ------ | --------------------------------- |
| `tests::bench_fs`                  | vanilla file system | False  | 1                                 |
| `tests::bench_git`                 | `git`               | False  | 1                                 |
| `tests::bench_cache_fs`            | vanilla file system | True   | 1                                 |
| `tests::bench_cache_git`           | `git`               | True   | 1                                 |
| `tests::bench_fs_100_files`        | vanilla file system | False  | 100                               |
| `tests::bench_git_100_files`       | `git`               | False  | 100                               |
| `tests::bench_cache_fs_100_files`  | vanilla file system | True   | 100                               |
| `tests::bench_cache_git_100_files` | `git`               | True   | 100                               |

See [results](#results)

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

Run on Intel Core i7-9750h, NVMe SSD(read: 1,700MB/s), 32GB DDR4 @2667 MHz

```bash
➜  git-bench git:(master) ✗ cargo bench
    Finished bench [optimized] target(s) in 0.34s
     Running unittests (target/release/deps/git_bench-91a556763cda354f)

running 8 tests
test tests::bench_cache_fs            ... bench:          20 ns/iter (+/- 0)
test tests::bench_cache_fs_100_files  ... bench:       2,214 ns/iter (+/- 105)
test tests::bench_cache_git           ... bench:          20 ns/iter (+/- 0)
test tests::bench_cache_git_100_files ... bench:       2,232 ns/iter (+/- 138)
test tests::bench_fs                  ... bench:       3,810 ns/iter (+/- 159)
test tests::bench_fs_100_files        ... bench:     397,494 ns/iter (+/- 42,564)
test tests::bench_git                 ... bench:      67,731 ns/iter (+/- 1,146)
test tests::bench_git_100_files       ... bench:   7,100,108 ns/iter (+/- 149,560)

test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 0 filtered out; finished in 23.01s
```
