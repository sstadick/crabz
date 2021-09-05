#!/usr/bin/bash
set -eo pipefail

test_data="$1"
cp "${test_data}" ./data.txt

cargo clean
cargo build --release
./target/release/crabz -f mgzip -p 32 -l 3 ./data.txt > ./bdata.3.txt.gz
./target/release/crabz -f mgzip -p 32 -l 6 ./data.txt > ./bdata.6.txt.gz
./target/release/crabz -f mgzip -p 32 -l 9 ./data.txt > ./bdata.9.txt.gz
./target/release/crabz -f mgzip -p 32 -l 12 ./data.txt > ./bdata.12.txt.gz

gzip -3 -c ./data.txt > ./data.3.txt.gz
gzip -6 -c ./data.txt > ./data.6.txt.gz
gzip -9 -c ./data.txt > ./data.9.txt.gz

# hyperfine \
#     --warmup 3 \
#     --runs 3 \
#     --export-markdown "block_compression.md" \
#     --parameter-list num_threads 1,2,4,8,16,32 \
#     --parameter-list comp_level 3,6,9 \
#     './target/release/crabz -p {num_threads} -f bgzf -l {comp_level} ./data.txt > ./data.out.txt.gz ' \
#     'bgzip -f -c -l {comp_level} -@ {num_threads} ./data.txt > ./data.out.txt.gz'

#   hyperfine \
#       --warmup 3 \
#       --runs 3 \
#       --export-markdown "block_decompression.md" \
#       --parameter-list comp_level 3,6,9 \
#       --parameter-list num_threads 1,2,4,8,16,32 \
#       './target/release/crabz -p {num_threads} -d -f bgzf ./bdata.{comp_level}.txt.gz > bdata.txt' \
#       'bgzip -d -@ {num_threads} -c ./bdata.{comp_level}.txt.gz > bdata.txt' \
#       'pigz -d -c ./bdata.{comp_level}.txt.gz > bdata.txt'

# hyperfine \
#     --warmup 3 \
#     --runs 3 \
#     --export-markdown "decompression_blocks_default.md" \
#     --parameter-list comp_level 3,6,9,12 \
#     --parameter-list num_threads 1,2,4,8,16,32 \
#     './target/release/crabz -p {num_threads} -d -f mgzip ./bdata.{comp_level}.txt.gz > data.txt' \
#     'pigz -d -c ./bdata.{comp_level}.txt.gz > data.txt'

# Compression Tests
for backend in "deflate_zlib_ng,libdeflate,snap_default" "deflate_zlib,libdeflate,snap_default" "deflate_rust,libdeflate,snap_default"; do
    cargo clean
    cargo build --release --no-default-features --features "$backend"

    hyperfine \
        --warmup 3 \
        --runs 3 \
        --export-markdown "compression_${backend}.md" \
        --parameter-list num_threads 1,2,4,8,16,32 \
        --parameter-list comp_level 3,6,9 \
        './target/release/crabz -p {num_threads} -l {comp_level} -f gzip ./data.txt > ./data.out.txt.gz' \
        './target/release/crabz -p {num_threads} -l {comp_level} -f mgzip ./data.txt > ./data.out.txt.gz' \
        'pigz -c -p {num_threads} -{comp_level} ./data.txt > ./data.out.txt.gz'

  hyperfine \
      --warmup 3 \
      --runs 3 \
      --export-markdown "decompression_${backend}.md" \
      --parameter-list comp_level 3,6,9 \
      './target/release/crabz -d -f gzip ./data.{comp_level}.txt.gz > data.txt' \
      'pigz -d -c ./data.{comp_level}.txt.gz > data.txt'


done

rm data*
