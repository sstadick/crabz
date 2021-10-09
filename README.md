# 🦀 crabz


<p align="center">
  <a href="https://github.com/sstadick/crabz/actions?query=workflow%3ACheck"><img src="https://github.com/sstadick/crabz/workflows/Check/badge.svg" alt="Build Status"></a>
  <img src="https://img.shields.io/crates/l/crabz.svg" alt="license">
  <a href="https://crates.io/crates/crabz"><img src="https://img.shields.io/crates/v/crabz.svg?colorB=319e8c" alt="Version info"></a><br>
  Like pigz, but rust.
</p>

A cross platform, fast, compression and decompression tool.

## Synopsis

This is currently a proof of concept CLI tool using the [`gzp`](https://github.com/sstadick/gzp/) crate.

Supported formats:

- Gzip
- Zlib
- Mgzip
- BGZF
- Raw Deflate
- Snap

## Install

* Homebrew / Linuxbrew

```
brew tap sstadick/crabz
brew install crabz
```

* Debian (Ubuntu)
  
```
curl -LO https://github.com/sstadick/crabz/releases/download/<latest>/crabz-linux-amd64.deb
sudo dpkg -i crabz-linux-amd64.deb
```

* Cargo

```
cargo install crabz
```

* Conda

```
conda install -c conda-forge crabz
```


## Usage

```
❯ crabz -h              
Compress and decompress files

USAGE:
    crabz [FLAGS] [OPTIONS] [FILE]

FLAGS:
    -d, --decompress    Flag to switch to decompressing inputs. Note: this flag may change in future releases
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -l, --compression-level <compression-level>        Compression level [default: 6]
    -p, --compression-threads <compression-threads>
            Number of compression threads to use, or if decompressing a format that allow for multi-threaded
            decompression, the number to use. Note that > 4 threads for decompression doesn't seem to help [default:
            32]
    -f, --format <format>
            The format to use [default: gzip]  [possible values: gzip, bgzf, mgzip,
            zlib, deflate, snap]
    -o, --output <output>                              Output path to write to, empty or "-" to write to stdout

ARGS:
    <FILE>    Input file to read from, empty or "-" to read from stdin
```

## Benchmarks

These benchmarks use the data in `bench-data` catted together 100 times. Run with `bash ./benchmark.sh data.txt`.

Benchmark system specs: Ubuntu 20 AMD Ryzen 9 3950X 16-Core Processor w/ 64 GB DDR4 memory and 1TB NVMe Drive

`pigz` v2.4 installed via apt on Ubuntu

Takeaways:

- `crabz` with `zlib` backend is pretty much identical to `pigz`
- `crabz` with `zlib-ng` backend is roughly 30-50% faster than `pigz`
- `crabz` with `rust` backend is roughly 5-10% faster than `pigz`

It is already known that `zlib-ng` is faster than `zlib`, so none of this is groundbreaking. However, I think `crabz` gets an
an edge due to the following:

- `crabz` with `deflate_rust` backend is using all Rust only code, which is in theory more secure / safe.
- `crabz` with `zlib-ng` is easier to install than `pigz` with a `zlib-ng` backend
- `crabz` supports more formats than `pigz`
- `crabz` is cross platform and can run on windows

With regards to block formats like Mgzip and BGZF, `crabz` is using `libdeflater` by default which excels at compressing and
decompression known-sized blocks. This makes block compression formats very fast at a small loss to the compression ratio.

Comparing `crabz` against tools like `bgzip`, which also defaults to `libdeflater` as a backend shows them within a few percent of
eachother.

As `crabz` is just a wrapper for the `gzp` library, the most exciting thing about these benchmarks is that `gzp` is on par with
best in class CLI tools for multi-threaded compression and decompression as a library.


### Flate2 zlib-ng backend

#### Compression


| Command                         |       Mean [s] | Min [s] | Max [s] |      Relative |
| :------------------------------ | -------------: | ------: | ------: | ------------: |
| `crabz -p 1 -c 3 < ./data.txt`  |  6.450 ± 0.069 |   6.328 |   6.540 |  16.86 ± 0.24 |
| `pigz -p 1 -3 < ./data.txt`     | 11.404 ± 0.152 |  11.186 |  11.717 |  29.81 ± 0.49 |
| `crabz -p 2 -c 3 < ./data.txt`  |  3.437 ± 0.017 |   3.418 |   3.461 |   8.98 ± 0.10 |
| `pigz -p 2 -3 < ./data.txt`     |  5.868 ± 0.031 |   5.826 |   5.927 |  15.34 ± 0.17 |
| `crabz -p 4 -c 3 < ./data.txt`  |  1.741 ± 0.008 |   1.729 |   1.752 |   4.55 ± 0.05 |
| `pigz -p 4 -3 < ./data.txt`     |  2.952 ± 0.008 |   2.939 |   2.960 |   7.72 ± 0.08 |
| `crabz -p 8 -c 3 < ./data.txt`  |  0.889 ± 0.004 |   0.882 |   0.895 |   2.32 ± 0.02 |
| `pigz -p 8 -3 < ./data.txt`     |  1.505 ± 0.008 |   1.493 |   1.520 |   3.93 ± 0.04 |
| `crabz -p 16 -c 3 < ./data.txt` |  0.485 ± 0.014 |   0.457 |   0.502 |   1.27 ± 0.04 |
| `pigz -p 16 -3 < ./data.txt`    |  0.775 ± 0.011 |   0.764 |   0.797 |   2.02 ± 0.04 |
| `crabz -p 32 -c 3 < ./data.txt` |  0.383 ± 0.004 |   0.375 |   0.388 |          1.00 |
| `pigz -p 32 -3 < ./data.txt`    |  0.699 ± 0.029 |   0.668 |   0.770 |   1.83 ± 0.08 |
| `crabz -p 1 -c 6 < ./data.txt`  | 10.367 ± 0.211 |  10.106 |  10.642 |  27.10 ± 0.61 |
| `pigz -p 1 -6 < ./data.txt`     | 26.734 ± 0.345 |  26.234 |  27.135 |  69.89 ± 1.12 |
| `crabz -p 2 -c 6 < ./data.txt`  |  5.366 ± 0.036 |   5.299 |   5.429 |  14.03 ± 0.16 |
| `pigz -p 2 -6 < ./data.txt`     | 13.589 ± 0.083 |  13.428 |  13.679 |  35.52 ± 0.40 |
| `crabz -p 4 -c 6 < ./data.txt`  |  2.719 ± 0.021 |   2.694 |   2.757 |   7.11 ± 0.09 |
| `pigz -p 4 -6 < ./data.txt`     |  6.887 ± 0.013 |   6.871 |   6.916 |  18.00 ± 0.17 |
| `crabz -p 8 -c 6 < ./data.txt`  |  1.381 ± 0.007 |   1.372 |   1.397 |   3.61 ± 0.04 |
| `pigz -p 8 -6 < ./data.txt`     |  3.479 ± 0.008 |   3.463 |   3.488 |   9.09 ± 0.09 |
| `crabz -p 16 -c 6 < ./data.txt` |  0.745 ± 0.022 |   0.727 |   0.804 |   1.95 ± 0.06 |
| `pigz -p 16 -6 < ./data.txt`    |  1.818 ± 0.036 |   1.765 |   1.874 |   4.75 ± 0.10 |
| `crabz -p 32 -c 6 < ./data.txt` |  0.549 ± 0.006 |   0.538 |   0.557 |   1.44 ± 0.02 |
| `pigz -p 32 -6 < ./data.txt`    |  1.187 ± 0.011 |   1.172 |   1.210 |   3.10 ± 0.04 |
| `crabz -p 1 -c 9 < ./data.txt`  | 30.114 ± 0.196 |  29.842 |  30.420 |  78.72 ± 0.90 |
| `pigz -p 1 -9 < ./data.txt`     | 51.369 ± 0.164 |  51.246 |  51.698 | 134.29 ± 1.33 |
| `crabz -p 2 -c 9 < ./data.txt`  | 15.371 ± 0.070 |  15.202 |  15.443 |  40.18 ± 0.42 |
| `pigz -p 2 -9 < ./data.txt`     | 26.452 ± 0.085 |  26.253 |  26.576 |  69.15 ± 0.69 |
| `crabz -p 4 -c 9 < ./data.txt`  |  7.729 ± 0.022 |   7.699 |   7.768 |  20.20 ± 0.20 |
| `pigz -p 4 -9 < ./data.txt`     | 13.365 ± 0.047 |  13.271 |  13.449 |  34.94 ± 0.35 |
| `crabz -p 8 -c 9 < ./data.txt`  |  3.901 ± 0.006 |   3.889 |   3.910 |  10.20 ± 0.10 |
| `pigz -p 8 -9 < ./data.txt`     |  6.749 ± 0.014 |   6.737 |   6.781 |  17.64 ± 0.17 |
| `crabz -p 16 -c 9 < ./data.txt` |  2.039 ± 0.024 |   1.997 |   2.071 |   5.33 ± 0.08 |
| `pigz -p 16 -9 < ./data.txt`    |  3.486 ± 0.054 |   3.426 |   3.574 |   9.11 ± 0.17 |
| `crabz -p 32 -c 9 < ./data.txt` |  1.337 ± 0.072 |   1.220 |   1.411 |   3.49 ± 0.19 |
| `pigz -p 32 -9 < ./data.txt`    |  2.203 ± 0.114 |   2.082 |   2.378 |   5.76 ± 0.30 |


#### Decompression

| Command                      |      Mean [s] | Min [s] | Max [s] |    Relative |
| :--------------------------- | ------------: | ------: | ------: | ----------: |
| `crabz -d < ./data.3.txt.gz` | 1.422 ± 0.010 |   1.411 |   1.437 | 1.03 ± 0.02 |
| `pigz -d < ./data.3.txt.gz`  | 1.674 ± 0.031 |   1.621 |   1.705 | 1.21 ± 0.03 |
| `crabz -d < ./data.6.txt.gz` | 1.403 ± 0.016 |   1.389 |   1.427 | 1.01 ± 0.02 |
| `pigz -d < ./data.6.txt.gz`  | 1.724 ± 0.026 |   1.697 |   1.766 | 1.24 ± 0.02 |
| `crabz -d < ./data.9.txt.gz` | 1.385 ± 0.018 |   1.359 |   1.416 |        1.00 |
| `pigz -d < ./data.9.txt.gz`  | 1.745 ± 0.044 |   1.684 |   1.797 | 1.26 ± 0.04 |


### Flate2 zlib backend

#### Compression

| Command                         |       Mean [s] | Min [s] | Max [s] |     Relative |
| :------------------------------ | -------------: | ------: | ------: | -----------: |
| `crabz -p 1 -c 3 < ./data.txt`  | 11.248 ± 0.247 |  11.085 |  11.532 | 20.23 ± 0.45 |
| `pigz -p 1 -3 < ./data.txt`     | 11.296 ± 0.171 |  11.104 |  11.434 | 20.32 ± 0.31 |
| `crabz -p 2 -c 3 < ./data.txt`  |  5.681 ± 0.040 |   5.645 |   5.725 | 10.22 ± 0.08 |
| `pigz -p 2 -3 < ./data.txt`     |  5.926 ± 0.015 |   5.916 |   5.944 | 10.66 ± 0.04 |
| `crabz -p 4 -c 3 < ./data.txt`  |  2.891 ± 0.007 |   2.883 |   2.895 |  5.20 ± 0.02 |
| `pigz -p 4 -3 < ./data.txt`     |  2.966 ± 0.013 |   2.955 |   2.980 |  5.34 ± 0.03 |
| `crabz -p 8 -c 3 < ./data.txt`  |  1.461 ± 0.003 |   1.459 |   1.465 |  2.63 ± 0.01 |
| `pigz -p 8 -3 < ./data.txt`     |  1.509 ± 0.004 |   1.505 |   1.512 |  2.71 ± 0.01 |
| `crabz -p 16 -c 3 < ./data.txt` |  0.784 ± 0.010 |   0.775 |   0.795 |  1.41 ± 0.02 |
| `pigz -p 16 -3 < ./data.txt`    |  0.772 ± 0.010 |   0.765 |   0.784 |  1.39 ± 0.02 |
| `crabz -p 32 -c 3 < ./data.txt` |  0.556 ± 0.002 |   0.554 |   0.557 |         1.00 |
| `pigz -p 32 -3 < ./data.txt`    |  0.743 ± 0.047 |   0.694 |   0.786 |  1.34 ± 0.08 |
| `crabz -p 1 -c 6 < ./data.txt`  | 26.366 ± 0.154 |  26.189 |  26.469 | 47.42 ± 0.31 |
| `pigz -p 1 -6 < ./data.txt`     | 26.688 ± 0.103 |  26.579 |  26.783 | 48.00 ± 0.23 |
| `crabz -p 2 -c 6 < ./data.txt`  | 13.443 ± 0.069 |  13.400 |  13.523 | 24.18 ± 0.14 |
| `pigz -p 2 -6 < ./data.txt`     | 13.605 ± 0.059 |  13.567 |  13.673 | 24.47 ± 0.13 |
| `crabz -p 4 -c 6 < ./data.txt`  |  6.833 ± 0.005 |   6.828 |   6.837 | 12.29 ± 0.03 |
| `pigz -p 4 -6 < ./data.txt`     |  6.866 ± 0.028 |   6.834 |   6.884 | 12.35 ± 0.06 |
| `crabz -p 8 -c 6 < ./data.txt`  |  3.446 ± 0.000 |   3.445 |   3.446 |  6.20 ± 0.02 |
| `pigz -p 8 -6 < ./data.txt`     |  3.482 ± 0.002 |   3.480 |   3.483 |  6.26 ± 0.02 |
| `crabz -p 16 -c 6 < ./data.txt` |  1.822 ± 0.012 |   1.813 |   1.835 |  3.28 ± 0.02 |
| `pigz -p 16 -6 < ./data.txt`    |  1.771 ± 0.004 |   1.767 |   1.776 |  3.19 ± 0.01 |
| `crabz -p 32 -c 6 < ./data.txt` |  1.178 ± 0.008 |   1.171 |   1.187 |  2.12 ± 0.02 |
| `pigz -p 32 -6 < ./data.txt`    |  1.184 ± 0.001 |   1.184 |   1.185 |  2.13 ± 0.01 |
| `crabz -p 1 -c 9 < ./data.txt`  | 52.122 ± 0.288 |  51.790 |  52.293 | 93.75 ± 0.58 |
| `pigz -p 1 -9 < ./data.txt`     | 53.031 ± 0.071 |  52.951 |  53.085 | 95.39 ± 0.29 |
| `crabz -p 2 -c 9 < ./data.txt`  | 26.287 ± 0.047 |  26.249 |  26.339 | 47.28 ± 0.15 |
| `pigz -p 2 -9 < ./data.txt`     | 26.409 ± 0.238 |  26.190 |  26.662 | 47.50 ± 0.45 |
| `crabz -p 4 -c 9 < ./data.txt`  | 13.373 ± 0.051 |  13.317 |  13.419 | 24.05 ± 0.11 |
| `pigz -p 4 -9 < ./data.txt`     | 13.414 ± 0.035 |  13.383 |  13.451 | 24.13 ± 0.09 |
| `crabz -p 8 -c 9 < ./data.txt`  |  6.733 ± 0.003 |   6.731 |   6.736 | 12.11 ± 0.03 |
| `pigz -p 8 -9 < ./data.txt`     |  6.763 ± 0.004 |   6.761 |   6.767 | 12.16 ± 0.03 |
| `crabz -p 16 -c 9 < ./data.txt` |  3.487 ± 0.034 |   3.450 |   3.517 |  6.27 ± 0.06 |
| `pigz -p 16 -9 < ./data.txt`    |  3.459 ± 0.021 |   3.434 |   3.473 |  6.22 ± 0.04 |
| `crabz -p 32 -c 9 < ./data.txt` |  2.088 ± 0.008 |   2.081 |   2.097 |  3.76 ± 0.02 |
| `pigz -p 32 -9 < ./data.txt`    |  2.107 ± 0.023 |   2.090 |   2.133 |  3.79 ± 0.04 |


#### Decompression

### Flate2 rust backend

#### Compression

| Command                         |       Mean [s] | Min [s] | Max [s] |     Relative |
| :------------------------------ | -------------: | ------: | ------: | -----------: |
| `crabz -p 1 -c 3 < ./data.txt`  | 10.167 ± 0.164 |  10.050 |  10.355 | 18.57 ± 0.33 |
| `pigz -p 1 -3 < ./data.txt`     | 11.338 ± 0.071 |  11.292 |  11.420 | 20.71 ± 0.21 |
| `crabz -p 2 -c 3 < ./data.txt`  |  4.912 ± 0.013 |   4.898 |   4.920 |  8.97 ± 0.08 |
| `pigz -p 2 -3 < ./data.txt`     |  5.876 ± 0.047 |   5.826 |   5.919 | 10.73 ± 0.12 |
| `crabz -p 4 -c 3 < ./data.txt`  |  2.463 ± 0.018 |   2.447 |   2.482 |  4.50 ± 0.05 |
| `pigz -p 4 -3 < ./data.txt`     |  2.967 ± 0.008 |   2.958 |   2.972 |  5.42 ± 0.05 |
| `crabz -p 8 -c 3 < ./data.txt`  |  1.255 ± 0.005 |   1.250 |   1.261 |  2.29 ± 0.02 |
| `pigz -p 8 -3 < ./data.txt`     |  1.509 ± 0.002 |   1.507 |   1.511 |  2.76 ± 0.02 |
| `crabz -p 16 -c 3 < ./data.txt` |  0.705 ± 0.030 |   0.673 |   0.731 |  1.29 ± 0.05 |
| `pigz -p 16 -3 < ./data.txt`    |  0.780 ± 0.015 |   0.768 |   0.797 |  1.42 ± 0.03 |
| `crabz -p 32 -c 3 < ./data.txt` |  0.547 ± 0.004 |   0.544 |   0.552 |         1.00 |
| `pigz -p 32 -3 < ./data.txt`    |  0.755 ± 0.025 |   0.726 |   0.771 |  1.38 ± 0.05 |
| `crabz -p 1 -c 6 < ./data.txt`  | 27.064 ± 0.288 |  26.863 |  27.394 | 49.44 ± 0.66 |
| `pigz -p 1 -6 < ./data.txt`     | 27.034 ± 0.090 |  26.938 |  27.117 | 49.38 ± 0.43 |
| `crabz -p 2 -c 6 < ./data.txt`  | 12.400 ± 0.083 |  12.321 |  12.487 | 22.65 ± 0.24 |
| `pigz -p 2 -6 < ./data.txt`     | 13.619 ± 0.074 |  13.558 |  13.702 | 24.88 ± 0.24 |
| `crabz -p 4 -c 6 < ./data.txt`  |  6.279 ± 0.023 |   6.263 |   6.305 | 11.47 ± 0.10 |
| `pigz -p 4 -6 < ./data.txt`     |  6.879 ± 0.020 |   6.867 |   6.901 | 12.57 ± 0.11 |
| `crabz -p 8 -c 6 < ./data.txt`  |  3.189 ± 0.010 |   3.178 |   3.198 |  5.83 ± 0.05 |
| `pigz -p 8 -6 < ./data.txt`     |  3.477 ± 0.007 |   3.470 |   3.483 |  6.35 ± 0.05 |
| `crabz -p 16 -c 6 < ./data.txt` |  1.756 ± 0.015 |   1.740 |   1.771 |  3.21 ± 0.04 |
| `pigz -p 16 -6 < ./data.txt`    |  1.799 ± 0.024 |   1.779 |   1.827 |  3.29 ± 0.05 |
| `crabz -p 32 -c 6 < ./data.txt` |  1.192 ± 0.011 |   1.183 |   1.205 |  2.18 ± 0.03 |
| `pigz -p 32 -6 < ./data.txt`    |  1.196 ± 0.016 |   1.183 |   1.214 |  2.19 ± 0.03 |
| `crabz -p 1 -c 9 < ./data.txt`  | 44.907 ± 0.283 |  44.585 |  45.116 | 82.03 ± 0.84 |
| `pigz -p 1 -9 < ./data.txt`     | 53.109 ± 1.049 |  52.373 |  54.311 | 97.02 ± 2.07 |
| `crabz -p 2 -c 9 < ./data.txt`  | 19.977 ± 0.159 |  19.819 |  20.136 | 36.49 ± 0.41 |
| `pigz -p 2 -9 < ./data.txt`     | 26.562 ± 0.134 |  26.407 |  26.643 | 48.52 ± 0.46 |
| `crabz -p 4 -c 9 < ./data.txt`  | 10.397 ± 0.484 |  10.070 |  10.953 | 18.99 ± 0.90 |
| `pigz -p 4 -9 < ./data.txt`     | 13.346 ± 0.040 |  13.300 |  13.372 | 24.38 ± 0.21 |
| `crabz -p 8 -c 9 < ./data.txt`  |  5.100 ± 0.021 |   5.076 |   5.114 |  9.32 ± 0.08 |
| `pigz -p 8 -9 < ./data.txt`     |  6.754 ± 0.016 |   6.736 |   6.767 | 12.34 ± 0.10 |
| `crabz -p 16 -c 9 < ./data.txt` |  2.716 ± 0.014 |   2.708 |   2.732 |  4.96 ± 0.05 |
| `pigz -p 16 -9 < ./data.txt`    |  3.444 ± 0.038 |   3.420 |   3.487 |  6.29 ± 0.09 |
| `crabz -p 32 -c 9 < ./data.txt` |  1.747 ± 0.009 |   1.740 |   1.758 |  3.19 ± 0.03 |
| `pigz -p 32 -9 < ./data.txt`    |  2.086 ± 0.008 |   2.077 |   2.093 |  3.81 ± 0.03 |


#### Decompression

| Command                      |      Mean [s] | Min [s] | Max [s] |    Relative |
| :--------------------------- | ------------: | ------: | ------: | ----------: |
| `crabz -d < ./data.3.txt.gz` | 1.599 ± 0.014 |   1.573 |   1.615 |        1.00 |
| `pigz -d < ./data.3.txt.gz`  | 1.696 ± 0.020 |   1.654 |   1.725 | 1.06 ± 0.02 |
| `crabz -d < ./data.6.txt.gz` | 1.615 ± 0.012 |   1.586 |   1.626 | 1.01 ± 0.01 |
| `pigz -d < ./data.6.txt.gz`  | 1.760 ± 0.030 |   1.687 |   1.797 | 1.10 ± 0.02 |
| `crabz -d < ./data.9.txt.gz` | 1.613 ± 0.014 |   1.596 |   1.641 | 1.01 ± 0.01 |
| `pigz -d < ./data.9.txt.gz`  | 1.767 ± 0.012 |   1.748 |   1.787 | 1.11 ± 0.01 |

### Block Formats with libdeflater

#### Decompression

| Command                                                |      Mean [s] | Min [s] | Max [s] |    Relative |
| :----------------------------------------------------- | ------------: | ------: | ------: | ----------: |
| `crabz -p 1 -d -f mgzip ./bdata.3.txt.gz > data.txt`   | 1.221 ± 0.164 |   1.073 |   1.397 | 2.32 ± 0.31 |
| `pigz -d -c ./bdata.3.txt.gz > data.txt`               | 2.415 ± 0.063 |   2.347 |   2.472 | 4.58 ± 0.14 |
| `crabz -p 1 -d -f mgzip ./bdata.6.txt.gz > data.txt`   | 1.256 ± 0.063 |   1.200 |   1.325 | 2.38 ± 0.13 |
| `pigz -d -c ./bdata.6.txt.gz > data.txt`               | 2.513 ± 0.052 |   2.467 |   2.569 | 4.77 ± 0.13 |
| `crabz -p 1 -d -f mgzip ./bdata.9.txt.gz > data.txt`   | 1.147 ± 0.065 |   1.094 |   1.219 | 2.18 ± 0.13 |
| `pigz -d -c ./bdata.9.txt.gz > data.txt`               | 2.394 ± 0.118 |   2.262 |   2.488 | 4.54 ± 0.24 |
| `crabz -p 1 -d -f mgzip ./bdata.12.txt.gz > data.txt`  | 1.165 ± 0.074 |   1.106 |   1.248 | 2.21 ± 0.15 |
| `pigz -d -c ./bdata.12.txt.gz > data.txt`              | 2.457 ± 0.067 |   2.408 |   2.534 | 4.66 ± 0.15 |
| `crabz -p 2 -d -f mgzip ./bdata.3.txt.gz > data.txt`   | 0.634 ± 0.008 |   0.628 |   0.642 | 1.20 ± 0.03 |
| `pigz -d -c ./bdata.3.txt.gz > data.txt`               | 2.379 ± 0.012 |   2.368 |   2.391 | 4.51 ± 0.08 |
| `crabz -p 2 -d -f mgzip ./bdata.6.txt.gz > data.txt`   | 0.645 ± 0.015 |   0.629 |   0.658 | 1.22 ± 0.03 |
| `pigz -d -c ./bdata.6.txt.gz > data.txt`               | 2.438 ± 0.073 |   2.356 |   2.497 | 4.62 ± 0.16 |
| `crabz -p 2 -d -f mgzip ./bdata.9.txt.gz > data.txt`   | 0.659 ± 0.015 |   0.644 |   0.674 | 1.25 ± 0.04 |
| `pigz -d -c ./bdata.9.txt.gz > data.txt`               | 2.451 ± 0.075 |   2.400 |   2.538 | 4.65 ± 0.16 |
| `crabz -p 2 -d -f mgzip ./bdata.12.txt.gz > data.txt`  | 0.656 ± 0.015 |   0.647 |   0.673 | 1.24 ± 0.04 |
| `pigz -d -c ./bdata.12.txt.gz > data.txt`              | 2.450 ± 0.045 |   2.412 |   2.500 | 4.65 ± 0.12 |
| `crabz -p 4 -d -f mgzip ./bdata.3.txt.gz > data.txt`   | 0.577 ± 0.024 |   0.554 |   0.603 | 1.10 ± 0.05 |
| `pigz -d -c ./bdata.3.txt.gz > data.txt`               | 2.459 ± 0.052 |   2.420 |   2.518 | 4.66 ± 0.13 |
| `crabz -p 4 -d -f mgzip ./bdata.6.txt.gz > data.txt`   | 0.559 ± 0.024 |   0.531 |   0.576 | 1.06 ± 0.05 |
| `pigz -d -c ./bdata.6.txt.gz > data.txt`               | 2.538 ± 0.044 |   2.502 |   2.587 | 4.81 ± 0.12 |
| `crabz -p 4 -d -f mgzip ./bdata.9.txt.gz > data.txt`   | 0.552 ± 0.011 |   0.539 |   0.560 | 1.05 ± 0.03 |
| `pigz -d -c ./bdata.9.txt.gz > data.txt`               | 2.402 ± 0.018 |   2.385 |   2.420 | 4.56 ± 0.08 |
| `crabz -p 4 -d -f mgzip ./bdata.12.txt.gz > data.txt`  | 0.592 ± 0.040 |   0.546 |   0.616 | 1.12 ± 0.08 |
| `pigz -d -c ./bdata.12.txt.gz > data.txt`              | 2.525 ± 0.038 |   2.484 |   2.558 | 4.79 ± 0.11 |
| `crabz -p 8 -d -f mgzip ./bdata.3.txt.gz > data.txt`   | 0.563 ± 0.013 |   0.548 |   0.571 | 1.07 ± 0.03 |
| `pigz -d -c ./bdata.3.txt.gz > data.txt`               | 2.490 ± 0.126 |   2.369 |   2.621 | 4.72 ± 0.25 |
| `crabz -p 8 -d -f mgzip ./bdata.6.txt.gz > data.txt`   | 0.552 ± 0.018 |   0.533 |   0.569 | 1.05 ± 0.04 |
| `pigz -d -c ./bdata.6.txt.gz > data.txt`               | 2.531 ± 0.115 |   2.417 |   2.647 | 4.80 ± 0.23 |
| `crabz -p 8 -d -f mgzip ./bdata.9.txt.gz > data.txt`   | 0.603 ± 0.029 |   0.583 |   0.636 | 1.14 ± 0.06 |
| `pigz -d -c ./bdata.9.txt.gz > data.txt`               | 2.483 ± 0.042 |   2.435 |   2.515 | 4.71 ± 0.11 |
| `crabz -p 8 -d -f mgzip ./bdata.12.txt.gz > data.txt`  | 0.527 ± 0.009 |   0.519 |   0.537 |        1.00 |
| `pigz -d -c ./bdata.12.txt.gz > data.txt`              | 2.524 ± 0.093 |   2.417 |   2.583 | 4.79 ± 0.19 |
| `crabz -p 16 -d -f mgzip ./bdata.3.txt.gz > data.txt`  | 0.603 ± 0.058 |   0.551 |   0.665 | 1.14 ± 0.11 |
| `pigz -d -c ./bdata.3.txt.gz > data.txt`               | 2.392 ± 0.007 |   2.384 |   2.397 | 4.54 ± 0.08 |
| `crabz -p 16 -d -f mgzip ./bdata.6.txt.gz > data.txt`  | 0.611 ± 0.065 |   0.565 |   0.686 | 1.16 ± 0.13 |
| `pigz -d -c ./bdata.6.txt.gz > data.txt`               | 2.593 ± 0.148 |   2.427 |   2.712 | 4.92 ± 0.29 |
| `crabz -p 16 -d -f mgzip ./bdata.9.txt.gz > data.txt`  | 0.564 ± 0.027 |   0.541 |   0.594 | 1.07 ± 0.05 |
| `pigz -d -c ./bdata.9.txt.gz > data.txt`               | 2.426 ± 0.023 |   2.404 |   2.450 | 4.60 ± 0.09 |
| `crabz -p 16 -d -f mgzip ./bdata.12.txt.gz > data.txt` | 0.601 ± 0.020 |   0.582 |   0.623 | 1.14 ± 0.04 |
| `pigz -d -c ./bdata.12.txt.gz > data.txt`              | 2.528 ± 0.022 |   2.507 |   2.550 | 4.80 ± 0.09 |
| `crabz -p 32 -d -f mgzip ./bdata.3.txt.gz > data.txt`  | 0.595 ± 0.019 |   0.577 |   0.614 | 1.13 ± 0.04 |
| `pigz -d -c ./bdata.3.txt.gz > data.txt`               | 2.544 ± 0.107 |   2.422 |   2.621 | 4.83 ± 0.22 |
| `crabz -p 32 -d -f mgzip ./bdata.6.txt.gz > data.txt`  | 0.601 ± 0.021 |   0.586 |   0.626 | 1.14 ± 0.05 |
| `pigz -d -c ./bdata.6.txt.gz > data.txt`               | 2.519 ± 0.114 |   2.435 |   2.649 | 4.78 ± 0.23 |
| `crabz -p 32 -d -f mgzip ./bdata.9.txt.gz > data.txt`  | 0.565 ± 0.023 |   0.539 |   0.579 | 1.07 ± 0.05 |
| `pigz -d -c ./bdata.9.txt.gz > data.txt`               | 2.487 ± 0.064 |   2.415 |   2.540 | 4.72 ± 0.15 |
| `crabz -p 32 -d -f mgzip ./bdata.12.txt.gz > data.txt` | 0.557 ± 0.013 |   0.548 |   0.571 | 1.06 ± 0.03 |
| `pigz -d -c ./bdata.12.txt.gz > data.txt`              | 2.505 ± 0.105 |   2.442 |   2.626 | 4.75 ± 0.22 |


#### `crabz`, `pigz`, and `bgzip`

These benchmarks were run on the `all_train.csv` data found [here](https://archive.ics.uci.edu/ml/machine-learning-databases/00347/all_train.csv.gz)

#### Compression

| Command                                                        |        Mean [s] | Min [s] | Max [s] |     Relative |
| :------------------------------------------------------------- | --------------: | ------: | ------: | -----------: |
| `crabz -p 2 -P 0 -l 2 -f bgzf ./data.txt > ./data.out.txt.gz`  |  15.837 ± 0.137 |  15.688 |  15.959 |  5.52 ± 0.13 |
| `bgzip -c -@ 2 -l 2 ./data.txt > ./data.out.txt.gz`            |  19.471 ± 0.178 |  19.268 |  19.602 |  6.78 ± 0.16 |
| `crabz -p 2 -P 0 -l 2 -f gzip ./data.txt > ./data.out.txt.gz`  |  19.723 ± 0.632 |  19.285 |  20.448 |  6.87 ± 0.26 |
| `pigz -c -p 2 -2 ./data.txt > ./data.out.txt.gz`               |  32.249 ± 0.024 |  32.226 |  32.274 | 11.24 ± 0.24 |
| `crabz -p 4 -P 0 -l 2 -f bgzf ./data.txt > ./data.out.txt.gz`  |   8.601 ± 0.538 |   8.040 |   9.113 |  3.00 ± 0.20 |
| `bgzip -c -@ 4 -l 2 ./data.txt > ./data.out.txt.gz`            |  10.953 ± 0.033 |  10.929 |  10.990 |  3.82 ± 0.08 |
| `crabz -p 4 -P 0 -l 2 -f gzip ./data.txt > ./data.out.txt.gz`  |  10.887 ± 0.584 |  10.236 |  11.364 |  3.79 ± 0.22 |
| `pigz -c -p 4 -2 ./data.txt > ./data.out.txt.gz`               |  16.493 ± 0.323 |  16.257 |  16.861 |  5.75 ± 0.17 |
| `crabz -p 8 -P 0 -l 2 -f bgzf ./data.txt > ./data.out.txt.gz`  |   5.206 ± 0.372 |   4.780 |   5.464 |  1.81 ± 0.14 |
| `bgzip -c -@ 8 -l 2 ./data.txt > ./data.out.txt.gz`            |   6.920 ± 0.033 |   6.893 |   6.957 |  2.41 ± 0.05 |
| `crabz -p 8 -P 0 -l 2 -f gzip ./data.txt > ./data.out.txt.gz`  |   5.893 ± 0.135 |   5.777 |   6.041 |  2.05 ± 0.06 |
| `pigz -c -p 8 -2 ./data.txt > ./data.out.txt.gz`               |   8.974 ± 0.467 |   8.553 |   9.477 |  3.13 ± 0.18 |
| `crabz -p 16 -P 0 -l 2 -f bgzf ./data.txt > ./data.out.txt.gz` |   2.870 ± 0.061 |   2.816 |   2.936 |         1.00 |
| `bgzip -c -@ 16 -l 2 ./data.txt > ./data.out.txt.gz`           |   5.124 ± 0.107 |   5.040 |   5.244 |  1.79 ± 0.05 |
| `crabz -p 16 -P 0 -l 2 -f gzip ./data.txt > ./data.out.txt.gz` |   4.250 ± 0.323 |   3.933 |   4.579 |  1.48 ± 0.12 |
| `pigz -c -p 16 -2 ./data.txt > ./data.out.txt.gz`              |   4.767 ± 0.223 |   4.513 |   4.933 |  1.66 ± 0.09 |
| `crabz -p 32 -P 0 -l 2 -f bgzf ./data.txt > ./data.out.txt.gz` |   3.669 ± 0.303 |   3.320 |   3.865 |  1.28 ± 0.11 |
| `bgzip -c -@ 32 -l 2 ./data.txt > ./data.out.txt.gz`           |   4.676 ± 0.038 |   4.632 |   4.701 |  1.63 ± 0.04 |
| `crabz -p 32 -P 0 -l 2 -f gzip ./data.txt > ./data.out.txt.gz` |   4.324 ± 0.246 |   4.143 |   4.605 |  1.51 ± 0.09 |
| `pigz -c -p 32 -2 ./data.txt > ./data.out.txt.gz`              |   5.854 ± 0.070 |   5.795 |   5.931 |  2.04 ± 0.05 |
| `crabz -p 2 -P 0 -l 6 -f bgzf ./data.txt > ./data.out.txt.gz`  |  27.696 ± 0.147 |  27.593 |  27.864 |  9.65 ± 0.21 |
| `bgzip -c -@ 2 -l 6 ./data.txt > ./data.out.txt.gz`            |  30.961 ± 0.446 |  30.446 |  31.231 | 10.79 ± 0.28 |
| `crabz -p 2 -P 0 -l 6 -f gzip ./data.txt > ./data.out.txt.gz`  |  36.229 ± 0.175 |  36.092 |  36.427 | 12.62 ± 0.27 |
| `pigz -c -p 2 -6 ./data.txt > ./data.out.txt.gz`               |  97.175 ± 0.571 |  96.743 |  97.823 | 33.86 ± 0.74 |
| `crabz -p 4 -P 0 -l 6 -f bgzf ./data.txt > ./data.out.txt.gz`  |  14.802 ± 0.436 |  14.316 |  15.159 |  5.16 ± 0.19 |
| `bgzip -c -@ 4 -l 6 ./data.txt > ./data.out.txt.gz`            |  16.927 ± 0.130 |  16.789 |  17.048 |  5.90 ± 0.13 |
| `crabz -p 4 -P 0 -l 6 -f gzip ./data.txt > ./data.out.txt.gz`  |  19.192 ± 0.675 |  18.629 |  19.940 |  6.69 ± 0.27 |
| `pigz -c -p 4 -6 ./data.txt > ./data.out.txt.gz`               |  49.305 ± 0.114 |  49.203 |  49.429 | 17.18 ± 0.37 |
| `crabz -p 8 -P 0 -l 6 -f bgzf ./data.txt > ./data.out.txt.gz`  |   7.833 ± 0.065 |   7.784 |   7.907 |  2.73 ± 0.06 |
| `bgzip -c -@ 8 -l 6 ./data.txt > ./data.out.txt.gz`            |   9.858 ± 0.105 |   9.739 |   9.939 |  3.43 ± 0.08 |
| `crabz -p 8 -P 0 -l 6 -f gzip ./data.txt > ./data.out.txt.gz`  |  10.417 ± 0.979 |   9.626 |  11.511 |  3.63 ± 0.35 |
| `pigz -c -p 8 -6 ./data.txt > ./data.out.txt.gz`               |  25.276 ± 0.170 |  25.083 |  25.404 |  8.81 ± 0.20 |
| `crabz -p 16 -P 0 -l 6 -f bgzf ./data.txt > ./data.out.txt.gz` |   4.704 ± 0.321 |   4.337 |   4.937 |  1.64 ± 0.12 |
| `bgzip -c -@ 16 -l 6 ./data.txt > ./data.out.txt.gz`           |   6.565 ± 0.155 |   6.429 |   6.734 |  2.29 ± 0.07 |
| `crabz -p 16 -P 0 -l 6 -f gzip ./data.txt > ./data.out.txt.gz` |   5.722 ± 0.320 |   5.530 |   6.092 |  1.99 ± 0.12 |
| `pigz -c -p 16 -6 ./data.txt > ./data.out.txt.gz`              |  13.673 ± 0.129 |  13.525 |  13.762 |  4.76 ± 0.11 |
| `crabz -p 32 -P 0 -l 6 -f bgzf ./data.txt > ./data.out.txt.gz` |   4.202 ± 0.213 |   3.957 |   4.328 |  1.46 ± 0.08 |
| `bgzip -c -@ 32 -l 6 ./data.txt > ./data.out.txt.gz`           |   5.538 ± 0.135 |   5.395 |   5.663 |  1.93 ± 0.06 |
| `crabz -p 32 -P 0 -l 6 -f gzip ./data.txt > ./data.out.txt.gz` |   5.488 ± 0.064 |   5.423 |   5.550 |  1.91 ± 0.05 |
| `pigz -c -p 32 -6 ./data.txt > ./data.out.txt.gz`              |   9.079 ± 0.286 |   8.808 |   9.379 |  3.16 ± 0.12 |
| `crabz -p 2 -P 0 -l 9 -f bgzf ./data.txt > ./data.out.txt.gz`  | 162.875 ± 0.100 | 162.778 | 162.977 | 56.75 ± 1.20 |
| `bgzip -c -@ 2 -l 9 ./data.txt > ./data.out.txt.gz`            | 172.428 ± 0.242 | 172.207 | 172.687 | 60.08 ± 1.27 |
| `crabz -p 2 -P 0 -l 9 -f gzip ./data.txt > ./data.out.txt.gz`  | 139.245 ± 0.270 | 138.974 | 139.514 | 48.52 ± 1.03 |
| `pigz -c -p 2 -9 ./data.txt > ./data.out.txt.gz`               | 209.645 ± 0.058 | 209.580 | 209.691 | 73.05 ± 1.55 |
| `crabz -p 4 -P 0 -l 9 -f bgzf ./data.txt > ./data.out.txt.gz`  |  84.624 ± 0.185 |  84.414 |  84.762 | 29.49 ± 0.63 |
| `bgzip -c -@ 4 -l 9 ./data.txt > ./data.out.txt.gz`            |  87.228 ± 0.232 |  87.053 |  87.492 | 30.39 ± 0.65 |
| `crabz -p 4 -P 0 -l 9 -f gzip ./data.txt > ./data.out.txt.gz`  |  72.339 ± 0.166 |  72.187 |  72.517 | 25.21 ± 0.54 |
| `pigz -c -p 4 -9 ./data.txt > ./data.out.txt.gz`               | 106.579 ± 0.236 | 106.307 | 106.731 | 37.14 ± 0.79 |
| `crabz -p 8 -P 0 -l 9 -f bgzf ./data.txt > ./data.out.txt.gz`  |  42.988 ± 0.130 |  42.905 |  43.138 | 14.98 ± 0.32 |
| `bgzip -c -@ 8 -l 9 ./data.txt > ./data.out.txt.gz`            |  44.550 ± 0.097 |  44.449 |  44.642 | 15.52 ± 0.33 |
| `crabz -p 8 -P 0 -l 9 -f gzip ./data.txt > ./data.out.txt.gz`  |  36.555 ± 0.030 |  36.521 |  36.579 | 12.74 ± 0.27 |
| `pigz -c -p 8 -9 ./data.txt > ./data.out.txt.gz`               |  54.047 ± 0.016 |  54.030 |  54.062 | 18.83 ± 0.40 |
| `crabz -p 16 -P 0 -l 9 -f bgzf ./data.txt > ./data.out.txt.gz` |  22.391 ± 0.234 |  22.154 |  22.623 |  7.80 ± 0.18 |
| `bgzip -c -@ 16 -l 9 ./data.txt > ./data.out.txt.gz`           |  24.041 ± 0.237 |  23.813 |  24.286 |  8.38 ± 0.20 |
| `crabz -p 16 -P 0 -l 9 -f gzip ./data.txt > ./data.out.txt.gz` |  19.285 ± 0.125 |  19.141 |  19.363 |  6.72 ± 0.15 |
| `pigz -c -p 16 -9 ./data.txt > ./data.out.txt.gz`              |  27.645 ± 0.078 |  27.579 |  27.731 |  9.63 ± 0.21 |
| `crabz -p 32 -P 0 -l 9 -f bgzf ./data.txt > ./data.out.txt.gz` |  15.148 ± 0.138 |  14.992 |  15.252 |  5.28 ± 0.12 |
| `bgzip -c -@ 32 -l 9 ./data.txt > ./data.out.txt.gz`           |  16.091 ± 0.193 |  15.874 |  16.243 |  5.61 ± 0.14 |
| `crabz -p 32 -P 0 -l 9 -f gzip ./data.txt > ./data.out.txt.gz` |  11.832 ± 0.168 |  11.637 |  11.930 |  4.12 ± 0.11 |
| `pigz -c -p 32 -9 ./data.txt > ./data.out.txt.gz`              |  16.912 ± 0.095 |  16.804 |  16.982 |  5.89 ± 0.13 |

#### Decompression

| Command                                                 |      Mean [s] | Min [s] | Max [s] |    Relative |
| :------------------------------------------------------ | ------------: | ------: | ------: | ----------: |
| `crabz -d -p 4 -f bgzf ./data.txt.gz > ./data.out.txt`  | 5.941 ± 0.172 |   5.745 |   6.070 | 1.11 ± 0.09 |
| `bgzip -d -c -@ 4  ./data.txt.gz > ./data.out.txt`      | 5.357 ± 0.407 |   4.925 |   5.734 |        1.00 |
| `crabz -d -p 8 -f bgzf ./data.txt.gz > ./data.out.txt`  | 5.569 ± 0.496 |   5.023 |   5.990 | 1.04 ± 0.12 |
| `bgzip -d -c -@ 8  ./data.txt.gz > ./data.out.txt`      | 5.867 ± 0.252 |   5.682 |   6.154 | 1.10 ± 0.10 |
| `crabz -d -p 16 -f bgzf ./data.txt.gz > ./data.out.txt` | 5.663 ± 0.240 |   5.506 |   5.939 | 1.06 ± 0.09 |
| `bgzip -d -c -@ 16  ./data.txt.gz > ./data.out.txt`     | 5.534 ± 0.124 |   5.416 |   5.663 | 1.03 ± 0.08 |

## TODOs

- Test with jemalloc
- Add some form of auto format detection, even just by file extension

