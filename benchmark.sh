#!/usr/bin/bash
set -eo pipefail

test_data="$1"
cp "${test_data}" ./data.txt

gzip -3 -c ./data.txt > ./data.3.txt.gz
gzip -6 -c ./data.txt > ./data.6.txt.gz
gzip -9 -c ./data.txt > ./data.9.txt.gz

# Compression Tests
for backend in "deflate_zlib_ng" "deflate_zlib" "deflate_rust"; do
    cargo clean
    cargo build --release --no-default-features --features "$backend"

    hyperfine \
        --warmup 3 \
        --runs 3 \
        --export-markdown "compression_${backend}.md" \
        --parameter-list num_threads 1,2,4,8,16,32 \
        --parameter-list comp_level 3,6,9 \
        './target/release/crabz -p {num_threads} -c {comp_level} < ./data.txt' \
        'pigz -p {num_threads} -{comp_level} < ./data.txt'

    
    hyperfine \
        --warmup 3 \
        --runs 10 \
        --export-markdown "decompression_${backend}.md" \
        --parameter-list comp_level 3,6,9 \
        './target/release/crabz -d < ./data.{comp_level}.txt.gz' \
        'pigz -d < ./data.{comp_level}.txt.gz'
done

rm data*