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


## Usage

```
❯ crabz -h              
crabz 0.5.3
Seth Stadick
Compress and decompress files

USAGE:
    crabz [FLAGS] [OPTIONS] [FILE]

FLAGS:
    -d, --decompress    Flag to switch to decompressing inputs. Note: this flag may change in future releases
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -c, --compression-level <compression-level>        Compression level [default: 3]
    -p, --compression-threads <compression-threads>    Number of compression threads to use [default: 32]
    -f, --format <format>
            The format to use [default: gzip]  [possible values: gzip, zlib, deflate,
            snap]
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


## TODOs

- Make homebrew release
- Test with jemalloc
- Add some form of auto format detection, even just by file extension
- Implemement something like mgzip


## More Bench DAta

## Compression

### Bgzip vs Crabz with deflate_zlib_ng,libdeflate,snap_default

| Command                                                    |        Mean [s] | Min [s] | Max [s] |     Relative |
| :--------------------------------------------------------- | --------------: | ------: | ------: | -----------: |
| `crabz -p 1 -f bgzf -l 3 ./data.txt > ./data.out.txt.gz `  |  30.742 ± 0.521 |  30.163 |  31.173 | 15.55 ± 1.15 |
| `bgzip -f -c -l 3 -@ 1 ./data.txt > ./data.out.txt.gz`     |  22.587 ± 0.442 |  22.296 |  23.096 | 11.42 ± 0.85 |
| `crabz -p 2 -f bgzf -l 3 ./data.txt > ./data.out.txt.gz `  |  10.993 ± 0.245 |  10.812 |  11.271 |  5.56 ± 0.42 |
| `bgzip -f -c -l 3 -@ 2 ./data.txt > ./data.out.txt.gz`     |  12.005 ± 0.399 |  11.607 |  12.404 |  6.07 ± 0.48 |
| `crabz -p 4 -f bgzf -l 3 ./data.txt > ./data.out.txt.gz `  |   5.987 ± 0.349 |   5.716 |   6.381 |  3.03 ± 0.28 |
| `bgzip -f -c -l 3 -@ 4 ./data.txt > ./data.out.txt.gz`     |   6.608 ± 0.314 |   6.267 |   6.885 |  3.34 ± 0.29 |
| `crabz -p 8 -f bgzf -l 3 ./data.txt > ./data.out.txt.gz `  |   3.652 ± 0.155 |   3.497 |   3.806 |  1.85 ± 0.15 |
| `bgzip -f -c -l 3 -@ 8 ./data.txt > ./data.out.txt.gz`     |   3.855 ± 0.389 |   3.539 |   4.290 |  1.95 ± 0.24 |
| `crabz -p 16 -f bgzf -l 3 ./data.txt > ./data.out.txt.gz ` |   1.977 ± 0.142 |   1.893 |   2.142 |         1.00 |
| `bgzip -f -c -l 3 -@ 16 ./data.txt > ./data.out.txt.gz`    |   3.137 ± 0.074 |   3.088 |   3.222 |  1.59 ± 0.12 |
| `crabz -p 32 -f bgzf -l 3 ./data.txt > ./data.out.txt.gz ` |   2.044 ± 0.085 |   1.987 |   2.141 |  1.03 ± 0.09 |
| `bgzip -f -c -l 3 -@ 32 ./data.txt > ./data.out.txt.gz`    |   2.711 ± 0.052 |   2.653 |   2.751 |  1.37 ± 0.10 |
| `crabz -p 1 -f bgzf -l 6 ./data.txt > ./data.out.txt.gz `  |  48.635 ± 0.225 |  48.417 |  48.866 | 24.60 ± 1.78 |
| `bgzip -f -c -l 6 -@ 1 ./data.txt > ./data.out.txt.gz`     |  36.315 ± 0.814 |  35.754 |  37.249 | 18.37 ± 1.39 |
| `crabz -p 2 -f bgzf -l 6 ./data.txt > ./data.out.txt.gz `  |  18.088 ± 0.157 |  17.928 |  18.242 |  9.15 ± 0.66 |
| `bgzip -f -c -l 6 -@ 2 ./data.txt > ./data.out.txt.gz`     |  18.858 ± 0.286 |  18.624 |  19.177 |  9.54 ± 0.70 |
| `crabz -p 4 -f bgzf -l 6 ./data.txt > ./data.out.txt.gz `  |   9.284 ± 0.061 |   9.214 |   9.319 |  4.70 ± 0.34 |
| `bgzip -f -c -l 6 -@ 4 ./data.txt > ./data.out.txt.gz`     |  10.030 ± 0.377 |   9.729 |  10.453 |  5.07 ± 0.41 |
| `crabz -p 8 -f bgzf -l 6 ./data.txt > ./data.out.txt.gz `  |   4.909 ± 0.041 |   4.868 |   4.950 |  2.48 ± 0.18 |
| `bgzip -f -c -l 6 -@ 8 ./data.txt > ./data.out.txt.gz`     |   5.575 ± 0.315 |   5.247 |   5.875 |  2.82 ± 0.26 |
| `crabz -p 16 -f bgzf -l 6 ./data.txt > ./data.out.txt.gz ` |   3.007 ± 0.200 |   2.776 |   3.130 |  1.52 ± 0.15 |
| `bgzip -f -c -l 6 -@ 16 ./data.txt > ./data.out.txt.gz`    |   3.668 ± 0.457 |   3.142 |   3.976 |  1.86 ± 0.27 |
| `crabz -p 32 -f bgzf -l 6 ./data.txt > ./data.out.txt.gz ` |   2.406 ± 0.126 |   2.321 |   2.551 |  1.22 ± 0.11 |
| `bgzip -f -c -l 6 -@ 32 ./data.txt > ./data.out.txt.gz`    |   3.206 ± 0.046 |   3.153 |   3.238 |  1.62 ± 0.12 |
| `crabz -p 1 -f bgzf -l 9 ./data.txt > ./data.out.txt.gz `  | 135.172 ± 2.452 | 132.890 | 137.764 | 68.37 ± 5.08 |
| `bgzip -f -c -l 9 -@ 1 ./data.txt > ./data.out.txt.gz`     | 188.373 ± 0.995 | 187.255 | 189.161 | 95.27 ± 6.88 |
| `crabz -p 2 -f bgzf -l 9 ./data.txt > ./data.out.txt.gz `  |  95.278 ± 0.563 |  94.722 |  95.847 | 48.19 ± 3.48 |
| `bgzip -f -c -l 9 -@ 2 ./data.txt > ./data.out.txt.gz`     |  94.182 ± 0.928 |  93.195 |  95.037 | 47.63 ± 3.46 |
| `crabz -p 4 -f bgzf -l 9 ./data.txt > ./data.out.txt.gz `  |  48.107 ± 0.090 |  48.034 |  48.207 | 24.33 ± 1.75 |
| `bgzip -f -c -l 9 -@ 4 ./data.txt > ./data.out.txt.gz`     |  48.203 ± 0.184 |  47.992 |  48.328 | 24.38 ± 1.76 |
| `crabz -p 8 -f bgzf -l 9 ./data.txt > ./data.out.txt.gz `  |  24.467 ± 0.062 |  24.405 |  24.529 | 12.37 ± 0.89 |
| `bgzip -f -c -l 9 -@ 8 ./data.txt > ./data.out.txt.gz`     |  24.692 ± 0.178 |  24.488 |  24.816 | 12.49 ± 0.90 |
| `crabz -p 16 -f bgzf -l 9 ./data.txt > ./data.out.txt.gz ` |  12.826 ± 0.226 |  12.677 |  13.086 |  6.49 ± 0.48 |
| `bgzip -f -c -l 9 -@ 16 ./data.txt > ./data.out.txt.gz`    |  12.608 ± 0.096 |  12.525 |  12.713 |  6.38 ± 0.46 |
| `crabz -p 32 -f bgzf -l 9 ./data.txt > ./data.out.txt.gz ` |   8.749 ± 0.144 |   8.583 |   8.832 |  4.42 ± 0.33 |
| `bgzip -f -c -l 9 -@ 32 ./data.txt > ./data.out.txt.gz`    |   8.380 ± 0.243 |   8.222 |   8.660 |  4.24 ± 0.33 |

### Pigz vs Crabz gzip / Crabz mgzip deflate_zlib,libdeflate,snap_default

| Command                                                    |        Mean [s] | Min [s] | Max [s] |       Relative |
| :--------------------------------------------------------- | --------------: | ------: | ------: | -------------: |
| `crabz -p 1 -l 3 ./data.txt > ./data.out.txt.gz`           |  30.314 ± 0.445 |  29.804 |  30.619 |   17.25 ± 1.52 |
| `crabz -p 1 -l 3 -f mgzip ./data.txt > ./data.out.txt.gz`  |  30.268 ± 0.637 |  29.650 |  30.923 |   17.22 ± 1.54 |
| `pigz -c -p 1 -3 ./data.txt > ./data.out.txt.gz`           |  51.839 ± 0.563 |  51.421 |  52.479 |   29.50 ± 2.59 |
| `crabz -p 2 -l 3 ./data.txt > ./data.out.txt.gz`           |  16.041 ± 0.178 |  15.899 |  16.241 |    9.13 ± 0.80 |
| `crabz -p 2 -l 3 -f mgzip ./data.txt > ./data.out.txt.gz`  |  10.792 ± 0.035 |  10.759 |  10.830 |    6.14 ± 0.53 |
| `pigz -c -p 2 -3 ./data.txt > ./data.out.txt.gz`           |  27.145 ± 0.228 |  26.914 |  27.371 |   15.45 ± 1.35 |
| `crabz -p 4 -l 3 ./data.txt > ./data.out.txt.gz`           |   8.219 ± 0.129 |   8.070 |   8.304 |    4.68 ± 0.41 |
| `crabz -p 4 -l 3 -f mgzip ./data.txt > ./data.out.txt.gz`  |   5.593 ± 0.140 |   5.431 |   5.675 |    3.18 ± 0.29 |
| `pigz -c -p 4 -3 ./data.txt > ./data.out.txt.gz`           |  13.746 ± 0.155 |  13.568 |  13.845 |    7.82 ± 0.69 |
| `crabz -p 8 -l 3 ./data.txt > ./data.out.txt.gz`           |   4.647 ± 0.223 |   4.393 |   4.811 |    2.64 ± 0.26 |
| `crabz -p 8 -l 3 -f mgzip ./data.txt > ./data.out.txt.gz`  |   3.067 ± 0.016 |   3.049 |   3.078 |    1.75 ± 0.15 |
| `pigz -c -p 8 -3 ./data.txt > ./data.out.txt.gz`           |   7.202 ± 0.014 |   7.186 |   7.210 |    4.10 ± 0.36 |
| `crabz -p 16 -l 3 ./data.txt > ./data.out.txt.gz`          |   2.809 ± 0.230 |   2.557 |   3.007 |    1.60 ± 0.19 |
| `crabz -p 16 -l 3 -f mgzip ./data.txt > ./data.out.txt.gz` |   2.127 ± 0.165 |   1.990 |   2.310 |    1.21 ± 0.14 |
| `pigz -c -p 16 -3 ./data.txt > ./data.out.txt.gz`          |   3.979 ± 0.171 |   3.871 |   4.175 |    2.26 ± 0.22 |
| `crabz -p 32 -l 3 ./data.txt > ./data.out.txt.gz`          |   2.130 ± 0.031 |   2.094 |   2.149 |    1.21 ± 0.11 |
| `crabz -p 32 -l 3 -f mgzip ./data.txt > ./data.out.txt.gz` |   1.757 ± 0.153 |   1.647 |   1.932 |           1.00 |
| `pigz -c -p 32 -3 ./data.txt > ./data.out.txt.gz`          |   3.672 ± 0.076 |   3.597 |   3.748 |    2.09 ± 0.19 |
| `crabz -p 1 -l 6 ./data.txt > ./data.out.txt.gz`           |  47.893 ± 0.384 |  47.590 |  48.324 |   27.25 ± 2.38 |
| `crabz -p 1 -l 6 -f mgzip ./data.txt > ./data.out.txt.gz`  |  48.691 ± 0.207 |  48.472 |  48.882 |   27.71 ± 2.41 |
| `pigz -c -p 1 -6 ./data.txt > ./data.out.txt.gz`           | 123.002 ± 0.526 | 122.692 | 123.609 |   69.99 ± 6.10 |
| `crabz -p 2 -l 6 ./data.txt > ./data.out.txt.gz`           |  24.674 ± 0.093 |  24.593 |  24.776 |   14.04 ± 1.22 |
| `crabz -p 2 -l 6 -f mgzip ./data.txt > ./data.out.txt.gz`  |  18.373 ± 0.318 |  18.078 |  18.710 |   10.45 ± 0.93 |
| `pigz -c -p 2 -6 ./data.txt > ./data.out.txt.gz`           |  61.963 ± 0.448 |  61.445 |  62.225 |   35.26 ± 3.08 |
| `crabz -p 4 -l 6 ./data.txt > ./data.out.txt.gz`           |  12.688 ± 0.045 |  12.661 |  12.740 |    7.22 ± 0.63 |
| `crabz -p 4 -l 6 -f mgzip ./data.txt > ./data.out.txt.gz`  |   9.563 ± 0.058 |   9.496 |   9.604 |    5.44 ± 0.47 |
| `pigz -c -p 4 -6 ./data.txt > ./data.out.txt.gz`           |  31.212 ± 0.097 |  31.099 |  31.273 |   17.76 ± 1.55 |
| `crabz -p 8 -l 6 ./data.txt > ./data.out.txt.gz`           |   6.598 ± 0.084 |   6.502 |   6.648 |    3.75 ± 0.33 |
| `crabz -p 8 -l 6 -f mgzip ./data.txt > ./data.out.txt.gz`  |   5.140 ± 0.137 |   5.025 |   5.292 |    2.93 ± 0.27 |
| `pigz -c -p 8 -6 ./data.txt > ./data.out.txt.gz`           |  16.118 ± 0.037 |  16.076 |  16.143 |    9.17 ± 0.80 |
| `crabz -p 16 -l 6 ./data.txt > ./data.out.txt.gz`          |   3.616 ± 0.072 |   3.543 |   3.687 |    2.06 ± 0.18 |
| `crabz -p 16 -l 6 -f mgzip ./data.txt > ./data.out.txt.gz` |   2.830 ± 0.045 |   2.801 |   2.882 |    1.61 ± 0.14 |
| `pigz -c -p 16 -6 ./data.txt > ./data.out.txt.gz`          |   8.678 ± 0.314 |   8.338 |   8.957 |    4.94 ± 0.47 |
| `crabz -p 32 -l 6 ./data.txt > ./data.out.txt.gz`          |   2.825 ± 0.008 |   2.819 |   2.835 |    1.61 ± 0.14 |
| `crabz -p 32 -l 6 -f mgzip ./data.txt > ./data.out.txt.gz` |   2.332 ± 0.010 |   2.322 |   2.342 |    1.33 ± 0.12 |
| `pigz -c -p 32 -6 ./data.txt > ./data.out.txt.gz`          |   5.745 ± 0.071 |   5.681 |   5.822 |    3.27 ± 0.29 |
| `crabz -p 1 -l 9 ./data.txt > ./data.out.txt.gz`           | 138.330 ± 0.527 | 137.729 | 138.714 |   78.71 ± 6.86 |
| `crabz -p 1 -l 9 -f mgzip ./data.txt > ./data.out.txt.gz`  | 137.268 ± 1.552 | 135.587 | 138.645 |   78.11 ± 6.86 |
| `pigz -c -p 1 -9 ./data.txt > ./data.out.txt.gz`           | 235.528 ± 2.933 | 233.612 | 238.904 | 134.02 ± 11.78 |
| `crabz -p 2 -l 9 ./data.txt > ./data.out.txt.gz`           |  69.526 ± 0.188 |  69.348 |  69.722 |   39.56 ± 3.44 |
| `crabz -p 2 -l 9 -f mgzip ./data.txt > ./data.out.txt.gz`  |  99.411 ± 0.216 |  99.168 |  99.580 |   56.57 ± 4.92 |
| `pigz -c -p 2 -9 ./data.txt > ./data.out.txt.gz`           | 120.630 ± 0.747 | 120.129 | 121.489 |   68.64 ± 5.99 |
| `crabz -p 4 -l 9 ./data.txt > ./data.out.txt.gz`           |  35.359 ± 0.117 |  35.226 |  35.441 |   20.12 ± 1.75 |
| `crabz -p 4 -l 9 -f mgzip ./data.txt > ./data.out.txt.gz`  |  50.238 ± 0.169 |  50.120 |  50.432 |   28.59 ± 2.49 |
| `pigz -c -p 4 -9 ./data.txt > ./data.out.txt.gz`           |  60.781 ± 0.263 |  60.477 |  60.939 |   34.59 ± 3.01 |
| `crabz -p 8 -l 9 ./data.txt > ./data.out.txt.gz`           |  18.045 ± 0.115 |  17.917 |  18.137 |   10.27 ± 0.90 |
| `crabz -p 8 -l 9 -f mgzip ./data.txt > ./data.out.txt.gz`  |  25.736 ± 0.143 |  25.624 |  25.898 |   14.64 ± 1.28 |
| `pigz -c -p 8 -9 ./data.txt > ./data.out.txt.gz`           |  30.999 ± 0.047 |  30.948 |  31.043 |   17.64 ± 1.54 |
| `crabz -p 16 -l 9 ./data.txt > ./data.out.txt.gz`          |   9.372 ± 0.184 |   9.164 |   9.513 |    5.33 ± 0.48 |
| `crabz -p 16 -l 9 -f mgzip ./data.txt > ./data.out.txt.gz` |  13.418 ± 0.280 |  13.220 |  13.738 |    7.64 ± 0.68 |
| `pigz -c -p 16 -9 ./data.txt > ./data.out.txt.gz`          |  16.000 ± 0.065 |  15.926 |  16.038 |    9.10 ± 0.79 |
| `crabz -p 32 -l 9 ./data.txt > ./data.out.txt.gz`          |   6.227 ± 0.279 |   5.906 |   6.408 |    3.54 ± 0.35 |
| `crabz -p 32 -l 9 -f mgzip ./data.txt > ./data.out.txt.gz` |   9.172 ± 0.103 |   9.054 |   9.238 |    5.22 ± 0.46 |
| `pigz -c -p 32 -9 ./data.txt > ./data.out.txt.gz`          |  10.029 ± 0.296 |   9.837 |  10.370 |    5.71 ± 0.52 |


## Decompression


### deflate_zlib_ng,libdeflate,snap_default


| Command                                                              |      Mean [s] | Min [s] | Max [s] |    Relative |
| :------------------------------------------------------------------- | ------------: | ------: | ------: | ----------: |
| `./target/release/crabz -p 1 -d -f gzip ./data.3.txt.gz > data.txt`  | 9.483 ± 0.502 |   8.955 |   9.953 | 1.11 ± 0.07 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              | 8.745 ± 0.093 |   8.645 |   8.828 | 1.03 ± 0.04 |
| `./target/release/crabz -p 1 -d -f gzip ./data.6.txt.gz > data.txt`  | 9.054 ± 0.050 |   9.017 |   9.111 | 1.06 ± 0.04 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              | 9.055 ± 0.130 |   8.929 |   9.188 | 1.06 ± 0.04 |
| `./target/release/crabz -p 1 -d -f gzip ./data.9.txt.gz > data.txt`  | 9.002 ± 0.516 |   8.497 |   9.528 | 1.06 ± 0.07 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              | 8.851 ± 0.268 |   8.542 |   9.021 | 1.04 ± 0.05 |
| `./target/release/crabz -p 2 -d -f gzip ./data.3.txt.gz > data.txt`  | 9.490 ± 0.332 |   9.171 |   9.833 | 1.11 ± 0.06 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              | 9.096 ± 0.452 |   8.634 |   9.537 | 1.07 ± 0.07 |
| `./target/release/crabz -p 2 -d -f gzip ./data.6.txt.gz > data.txt`  | 8.857 ± 0.494 |   8.288 |   9.172 | 1.04 ± 0.07 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              | 8.831 ± 0.394 |   8.377 |   9.078 | 1.04 ± 0.06 |
| `./target/release/crabz -p 2 -d -f gzip ./data.9.txt.gz > data.txt`  | 8.770 ± 0.445 |   8.260 |   9.076 | 1.03 ± 0.06 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              | 8.904 ± 0.195 |   8.684 |   9.056 | 1.04 ± 0.04 |
| `./target/release/crabz -p 4 -d -f gzip ./data.3.txt.gz > data.txt`  | 9.421 ± 0.086 |   9.371 |   9.520 | 1.10 ± 0.04 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              | 8.711 ± 0.081 |   8.617 |   8.759 | 1.02 ± 0.04 |
| `./target/release/crabz -p 4 -d -f gzip ./data.6.txt.gz > data.txt`  | 9.129 ± 0.023 |   9.105 |   9.151 | 1.07 ± 0.04 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              | 8.880 ± 0.380 |   8.441 |   9.122 | 1.04 ± 0.06 |
| `./target/release/crabz -p 4 -d -f gzip ./data.9.txt.gz > data.txt`  | 9.081 ± 0.499 |   8.593 |   9.589 | 1.06 ± 0.07 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              | 9.186 ± 0.487 |   8.733 |   9.701 | 1.08 ± 0.07 |
| `./target/release/crabz -p 8 -d -f gzip ./data.3.txt.gz > data.txt`  | 9.531 ± 0.185 |   9.422 |   9.745 | 1.12 ± 0.05 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              | 8.748 ± 0.512 |   8.289 |   9.300 | 1.03 ± 0.07 |
| `./target/release/crabz -p 8 -d -f gzip ./data.6.txt.gz > data.txt`  | 9.070 ± 0.273 |   8.785 |   9.329 | 1.06 ± 0.05 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              | 9.298 ± 0.524 |   8.938 |   9.899 | 1.09 ± 0.07 |
| `./target/release/crabz -p 8 -d -f gzip ./data.9.txt.gz > data.txt`  | 8.938 ± 0.060 |   8.874 |   8.993 | 1.05 ± 0.04 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              | 9.029 ± 0.613 |   8.440 |   9.664 | 1.06 ± 0.08 |
| `./target/release/crabz -p 16 -d -f gzip ./data.3.txt.gz > data.txt` | 9.352 ± 0.106 |   9.231 |   9.420 | 1.10 ± 0.04 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              | 8.529 ± 0.304 |   8.221 |   8.829 |        1.00 |
| `./target/release/crabz -p 16 -d -f gzip ./data.6.txt.gz > data.txt` | 9.066 ± 0.102 |   8.958 |   9.161 | 1.06 ± 0.04 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              | 8.824 ± 0.399 |   8.364 |   9.085 | 1.03 ± 0.06 |
| `./target/release/crabz -p 16 -d -f gzip ./data.9.txt.gz > data.txt` | 8.904 ± 0.522 |   8.350 |   9.385 | 1.04 ± 0.07 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              | 8.876 ± 0.264 |   8.571 |   9.045 | 1.04 ± 0.05 |
| `./target/release/crabz -p 32 -d -f gzip ./data.3.txt.gz > data.txt` | 9.391 ± 0.047 |   9.358 |   9.445 | 1.10 ± 0.04 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              | 8.789 ± 0.092 |   8.715 |   8.892 | 1.03 ± 0.04 |
| `./target/release/crabz -p 32 -d -f gzip ./data.6.txt.gz > data.txt` | 9.129 ± 0.029 |   9.110 |   9.162 | 1.07 ± 0.04 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              | 9.062 ± 0.091 |   8.959 |   9.132 | 1.06 ± 0.04 |
| `./target/release/crabz -p 32 -d -f gzip ./data.9.txt.gz > data.txt` | 8.946 ± 0.186 |   8.732 |   9.060 | 1.05 ± 0.04 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              | 8.981 ± 0.073 |   8.901 |   9.045 | 1.05 ± 0.04 |

### deflate_zlib,libdeflate,snap_default


| Command                                                              |       Mean [s] | Min [s] | Max [s] |    Relative |
| :------------------------------------------------------------------- | -------------: | ------: | ------: | ----------: |
| `./target/release/crabz -p 1 -d -f gzip ./data.3.txt.gz > data.txt`  | 11.076 ± 0.604 |  10.441 |  11.644 | 1.30 ± 0.08 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              |  8.760 ± 0.097 |   8.675 |   8.866 | 1.03 ± 0.03 |
| `./target/release/crabz -p 1 -d -f gzip ./data.6.txt.gz > data.txt`  | 11.123 ± 0.109 |  11.038 |  11.246 | 1.31 ± 0.04 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              |  8.761 ± 0.462 |   8.229 |   9.061 | 1.03 ± 0.06 |
| `./target/release/crabz -p 1 -d -f gzip ./data.9.txt.gz > data.txt`  | 10.792 ± 0.221 |  10.539 |  10.945 | 1.27 ± 0.04 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              |  9.024 ± 0.156 |   8.885 |   9.192 | 1.06 ± 0.03 |
| `./target/release/crabz -p 2 -d -f gzip ./data.3.txt.gz > data.txt`  | 11.044 ± 0.076 |  10.971 |  11.123 | 1.30 ± 0.04 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              |  8.710 ± 0.066 |   8.634 |   8.753 | 1.02 ± 0.03 |
| `./target/release/crabz -p 2 -d -f gzip ./data.6.txt.gz > data.txt`  | 10.944 ± 0.441 |  10.488 |  11.367 | 1.29 ± 0.06 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              |  9.354 ± 0.267 |   9.061 |   9.584 | 1.10 ± 0.04 |
| `./target/release/crabz -p 2 -d -f gzip ./data.9.txt.gz > data.txt`  | 10.894 ± 0.159 |  10.711 |  10.994 | 1.28 ± 0.04 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              |  9.040 ± 0.206 |   8.804 |   9.188 | 1.06 ± 0.04 |
| `./target/release/crabz -p 4 -d -f gzip ./data.3.txt.gz > data.txt`  | 11.125 ± 0.475 |  10.708 |  11.642 | 1.31 ± 0.07 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              |  8.711 ± 0.049 |   8.680 |   8.767 | 1.02 ± 0.03 |
| `./target/release/crabz -p 4 -d -f gzip ./data.6.txt.gz > data.txt`  | 10.969 ± 0.426 |  10.520 |  11.367 | 1.29 ± 0.06 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              |  9.138 ± 0.136 |   9.013 |   9.282 | 1.07 ± 0.03 |
| `./target/release/crabz -p 4 -d -f gzip ./data.9.txt.gz > data.txt`  | 10.983 ± 0.040 |  10.954 |  11.029 | 1.29 ± 0.03 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              |  8.854 ± 0.447 |   8.341 |   9.158 | 1.04 ± 0.06 |
| `./target/release/crabz -p 8 -d -f gzip ./data.3.txt.gz > data.txt`  | 10.790 ± 0.313 |  10.431 |  11.004 | 1.27 ± 0.05 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              |  8.512 ± 0.228 |   8.249 |   8.646 |        1.00 |
| `./target/release/crabz -p 8 -d -f gzip ./data.6.txt.gz > data.txt`  | 11.007 ± 0.029 |  10.975 |  11.031 | 1.29 ± 0.03 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              |  9.150 ± 0.382 |   8.831 |   9.573 | 1.08 ± 0.05 |
| `./target/release/crabz -p 8 -d -f gzip ./data.9.txt.gz > data.txt`  | 11.083 ± 0.126 |  11.002 |  11.228 | 1.30 ± 0.04 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              |  8.847 ± 0.265 |   8.542 |   9.012 | 1.04 ± 0.04 |
| `./target/release/crabz -p 16 -d -f gzip ./data.3.txt.gz > data.txt` | 10.925 ± 0.183 |  10.716 |  11.050 | 1.28 ± 0.04 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              |  9.057 ± 0.238 |   8.824 |   9.300 | 1.06 ± 0.04 |
| `./target/release/crabz -p 16 -d -f gzip ./data.6.txt.gz > data.txt` | 11.005 ± 0.632 |  10.334 |  11.589 | 1.29 ± 0.08 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              |  9.107 ± 0.082 |   9.017 |   9.176 | 1.07 ± 0.03 |
| `./target/release/crabz -p 16 -d -f gzip ./data.9.txt.gz > data.txt` | 10.768 ± 0.329 |  10.387 |  10.965 | 1.27 ± 0.05 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              |  8.961 ± 0.144 |   8.853 |   9.125 | 1.05 ± 0.03 |
| `./target/release/crabz -p 32 -d -f gzip ./data.3.txt.gz > data.txt` | 11.086 ± 0.023 |  11.060 |  11.105 | 1.30 ± 0.03 |
| `pigz -d -c ./data.3.txt.gz > data.txt`                              |  8.565 ± 0.248 |   8.285 |   8.758 | 1.01 ± 0.04 |
| `./target/release/crabz -p 32 -d -f gzip ./data.6.txt.gz > data.txt` | 10.947 ± 0.295 |  10.611 |  11.162 | 1.29 ± 0.05 |
| `pigz -d -c ./data.6.txt.gz > data.txt`                              |  8.953 ± 0.189 |   8.756 |   9.132 | 1.05 ± 0.04 |
| `./target/release/crabz -p 32 -d -f gzip ./data.9.txt.gz > data.txt` | 11.320 ± 0.386 |  10.887 |  11.627 | 1.33 ± 0.06 |
| `pigz -d -c ./data.9.txt.gz > data.txt`                              |  8.841 ± 0.416 |   8.361 |   9.093 | 1.04 ± 0.06 |


### Block decompress

| Command                                                                |       Mean [s] | Min [s] | Max [s] |    Relative |
| :--------------------------------------------------------------------- | -------------: | ------: | ------: | ----------: |
| `./target/release/crabz -p 1 -d -f bgzf ./bdata.3.txt.gz > bdata.txt`  |  5.126 ± 0.110 |   5.028 |   5.245 | 2.03 ± 0.06 |
| `bgzip -d -@ 1 -c ./bdata.3.txt.gz > bdata.txt`                        |  6.054 ± 0.517 |   5.672 |   6.642 | 2.40 ± 0.21 |
| `pigz -d -c ./bdata.3.txt.gz > bdata.txt`                              | 12.809 ± 0.625 |  12.165 |  13.414 | 5.08 ± 0.27 |
| `./target/release/crabz -p 1 -d -f bgzf ./bdata.6.txt.gz > bdata.txt`  |  5.415 ± 0.260 |   5.264 |   5.716 | 2.15 ± 0.11 |
| `bgzip -d -@ 1 -c ./bdata.6.txt.gz > bdata.txt`                        |  6.056 ± 0.187 |   5.944 |   6.272 | 2.40 ± 0.09 |
| `pigz -d -c ./bdata.6.txt.gz > bdata.txt`                              | 13.061 ± 0.069 |  12.984 |  13.114 | 5.18 ± 0.11 |
| `./target/release/crabz -p 1 -d -f bgzf ./bdata.9.txt.gz > bdata.txt`  |  5.558 ± 0.369 |   5.336 |   5.984 | 2.20 ± 0.15 |
| `bgzip -d -@ 1 -c ./bdata.9.txt.gz > bdata.txt`                        |  6.202 ± 0.206 |   6.042 |   6.435 | 2.46 ± 0.10 |
| `pigz -d -c ./bdata.9.txt.gz > bdata.txt`                              | 12.729 ± 0.596 |  12.231 |  13.390 | 5.05 ± 0.26 |
| `./target/release/crabz -p 2 -d -f bgzf ./bdata.3.txt.gz > bdata.txt`  |  3.112 ± 0.202 |   2.983 |   3.344 | 1.23 ± 0.08 |
| `bgzip -d -@ 2 -c ./bdata.3.txt.gz > bdata.txt`                        |  2.837 ± 0.459 |   2.395 |   3.311 | 1.13 ± 0.18 |
| `pigz -d -c ./bdata.3.txt.gz > bdata.txt`                              | 12.766 ± 0.288 |  12.519 |  13.082 | 5.06 ± 0.15 |
| `./target/release/crabz -p 2 -d -f bgzf ./bdata.6.txt.gz > bdata.txt`  |  3.115 ± 0.059 |   3.055 |   3.173 | 1.24 ± 0.03 |
| `bgzip -d -@ 2 -c ./bdata.6.txt.gz > bdata.txt`                        |  2.965 ± 0.293 |   2.773 |   3.303 | 1.18 ± 0.12 |
| `pigz -d -c ./bdata.6.txt.gz > bdata.txt`                              | 13.076 ± 0.099 |  12.967 |  13.162 | 5.19 ± 0.11 |
| `./target/release/crabz -p 2 -d -f bgzf ./bdata.9.txt.gz > bdata.txt`  |  3.235 ± 0.185 |   3.086 |   3.442 | 1.28 ± 0.08 |
| `bgzip -d -@ 2 -c ./bdata.9.txt.gz > bdata.txt`                        |  3.184 ± 0.393 |   2.730 |   3.413 | 1.26 ± 0.16 |
| `pigz -d -c ./bdata.9.txt.gz > bdata.txt`                              | 12.785 ± 0.319 |  12.418 |  12.993 | 5.07 ± 0.16 |
| `./target/release/crabz -p 4 -d -f bgzf ./bdata.3.txt.gz > bdata.txt`  |  2.880 ± 0.189 |   2.662 |   3.001 | 1.14 ± 0.08 |
| `bgzip -d -@ 4 -c ./bdata.3.txt.gz > bdata.txt`                        |  2.833 ± 0.225 |   2.576 |   2.995 | 1.12 ± 0.09 |
| `pigz -d -c ./bdata.3.txt.gz > bdata.txt`                              | 12.451 ± 0.332 |  12.071 |  12.684 | 4.94 ± 0.17 |
| `./target/release/crabz -p 4 -d -f bgzf ./bdata.6.txt.gz > bdata.txt`  |  2.862 ± 0.103 |   2.747 |   2.948 | 1.13 ± 0.05 |
| `bgzip -d -@ 4 -c ./bdata.6.txt.gz > bdata.txt`                        |  2.750 ± 0.072 |   2.678 |   2.823 | 1.09 ± 0.04 |
| `pigz -d -c ./bdata.6.txt.gz > bdata.txt`                              | 13.158 ± 0.425 |  12.910 |  13.648 | 5.22 ± 0.20 |
| `./target/release/crabz -p 4 -d -f bgzf ./bdata.9.txt.gz > bdata.txt`  |  2.840 ± 0.333 |   2.598 |   3.220 | 1.13 ± 0.13 |
| `bgzip -d -@ 4 -c ./bdata.9.txt.gz > bdata.txt`                        |  2.661 ± 0.008 |   2.655 |   2.670 | 1.06 ± 0.02 |
| `pigz -d -c ./bdata.9.txt.gz > bdata.txt`                              | 12.755 ± 0.247 |  12.479 |  12.956 | 5.06 ± 0.14 |
| `./target/release/crabz -p 8 -d -f bgzf ./bdata.3.txt.gz > bdata.txt`  |  2.866 ± 0.037 |   2.825 |   2.897 | 1.14 ± 0.03 |
| `bgzip -d -@ 8 -c ./bdata.3.txt.gz > bdata.txt`                        |  2.817 ± 0.113 |   2.700 |   2.926 | 1.12 ± 0.05 |
| `pigz -d -c ./bdata.3.txt.gz > bdata.txt`                              | 12.426 ± 0.301 |  12.090 |  12.671 | 4.93 ± 0.16 |
| `./target/release/crabz -p 8 -d -f bgzf ./bdata.6.txt.gz > bdata.txt`  |  2.776 ± 0.140 |   2.662 |   2.932 | 1.10 ± 0.06 |
| `bgzip -d -@ 8 -c ./bdata.6.txt.gz > bdata.txt`                        |  2.825 ± 0.340 |   2.610 |   3.218 | 1.12 ± 0.14 |
| `pigz -d -c ./bdata.6.txt.gz > bdata.txt`                              | 12.897 ± 0.157 |  12.718 |  13.012 | 5.12 ± 0.12 |
| `./target/release/crabz -p 8 -d -f bgzf ./bdata.9.txt.gz > bdata.txt`  |  2.834 ± 0.093 |   2.749 |   2.933 | 1.12 ± 0.04 |
| `bgzip -d -@ 8 -c ./bdata.9.txt.gz > bdata.txt`                        |  2.776 ± 0.050 |   2.720 |   2.815 | 1.10 ± 0.03 |
| `pigz -d -c ./bdata.9.txt.gz > bdata.txt`                              | 12.658 ± 0.461 |  12.131 |  12.983 | 5.02 ± 0.21 |
| `./target/release/crabz -p 16 -d -f bgzf ./bdata.3.txt.gz > bdata.txt` |  2.793 ± 0.104 |   2.682 |   2.888 | 1.11 ± 0.05 |
| `bgzip -d -@ 16 -c ./bdata.3.txt.gz > bdata.txt`                       |  2.521 ± 0.052 |   2.474 |   2.576 |        1.00 |
| `pigz -d -c ./bdata.3.txt.gz > bdata.txt`                              | 12.352 ± 0.493 |  11.784 |  12.650 | 4.90 ± 0.22 |
| `./target/release/crabz -p 16 -d -f bgzf ./bdata.6.txt.gz > bdata.txt` |  2.998 ± 0.376 |   2.577 |   3.301 | 1.19 ± 0.15 |
| `bgzip -d -@ 16 -c ./bdata.6.txt.gz > bdata.txt`                       |  2.843 ± 0.232 |   2.628 |   3.090 | 1.13 ± 0.09 |
| `pigz -d -c ./bdata.6.txt.gz > bdata.txt`                              | 13.099 ± 0.130 |  12.996 |  13.245 | 5.20 ± 0.12 |
| `./target/release/crabz -p 16 -d -f bgzf ./bdata.9.txt.gz > bdata.txt` |  2.779 ± 0.179 |   2.576 |   2.914 | 1.10 ± 0.07 |
| `bgzip -d -@ 16 -c ./bdata.9.txt.gz > bdata.txt`                       |  2.708 ± 0.200 |   2.529 |   2.923 | 1.07 ± 0.08 |
| `pigz -d -c ./bdata.9.txt.gz > bdata.txt`                              | 12.745 ± 0.424 |  12.265 |  13.068 | 5.05 ± 0.20 |
| `./target/release/crabz -p 32 -d -f bgzf ./bdata.3.txt.gz > bdata.txt` |  2.968 ± 0.109 |   2.881 |   3.090 | 1.18 ± 0.05 |
| `bgzip -d -@ 32 -c ./bdata.3.txt.gz > bdata.txt`                       |  2.747 ± 0.106 |   2.645 |   2.858 | 1.09 ± 0.05 |
| `pigz -d -c ./bdata.3.txt.gz > bdata.txt`                              | 12.584 ± 0.070 |  12.518 |  12.657 | 4.99 ± 0.11 |
| `./target/release/crabz -p 32 -d -f bgzf ./bdata.6.txt.gz > bdata.txt` |  2.891 ± 0.197 |   2.739 |   3.113 | 1.15 ± 0.08 |
| `bgzip -d -@ 32 -c ./bdata.6.txt.gz > bdata.txt`                       |  2.774 ± 0.053 |   2.714 |   2.817 | 1.10 ± 0.03 |
| `pigz -d -c ./bdata.6.txt.gz > bdata.txt`                              | 12.792 ± 0.411 |  12.392 |  13.213 | 5.07 ± 0.19 |
| `./target/release/crabz -p 32 -d -f bgzf ./bdata.9.txt.gz > bdata.txt` |  2.894 ± 0.199 |   2.670 |   3.050 | 1.15 ± 0.08 |
| `bgzip -d -@ 32 -c ./bdata.9.txt.gz > bdata.txt`                       |  2.868 ± 0.221 |   2.615 |   3.025 | 1.14 ± 0.09 |
| `pigz -d -c ./bdata.9.txt.gz > bdata.txt`                              | 12.665 ± 0.332 |  12.305 |  12.960 | 5.02 ± 0.17 |
