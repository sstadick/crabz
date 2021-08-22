# 🦀 crabz

Like pigz, but rust.

## Synopsis

This is currently a proof of concept CLI tool using the [`gzp`](https://github.com/sstadick/gzp/) crate.

## Benchmarks

This are very anecdotal. Data [here](https://archive.ics.uci.edu/ml/machine-learning-databases/00347/all_train.csv.gz).

Compiled with `cargo --release` and no tricks.

```bash
cat data.csv | /usr/bin/time crabz -c 3 > crabby.gz
79.34user 4.86system 0:06.52elapsed 1291%CPU (0avgtext+0avgdata 29868maxresident)k
0inputs+3632904outputs (0major+68221minor)pagefaults 0swaps
```

```bash
cat data.csv | /usr/bin/time pigz -3 > crabby.gz
120.65user 12.90system 0:11.36elapsed 1174%CPU (0avgtext+0avgdata 22904maxresident)k
0inputs+3763984outputs (0major+5352minor)pagefaults 0swaps
```

```bash
/usr/bin/time ./target/release/crabz -c 3 -o ./data.csv.gz data.csv
78.56user 3.72system 0:05.99elapsed 1373%CPU (0avgtext+0avgdata 30276maxresident)k
0inputs+3632904outputs (0major+64057minor)pagefaults 0swaps
```

```bash
/usr/bin/time pigz -3 data.csv 
121.20user 9.77system 0:08.01elapsed 1634%CPU (0avgtext+0avgdata 23240maxresident)k
0inputs+3763976outputs (0major+5360minor)pagefaults 0swaps
```

`crabz` seems to be about 20-30% faster than pigz.

This should be apples to apples with the same buffers, number of threads, and compression used.
The round trip md5sums tie out.