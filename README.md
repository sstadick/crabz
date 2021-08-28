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
- Raw Deflate
- Snap

## Install

```
cargo install crabz
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

- Test with jemalloc
