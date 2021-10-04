#!/usr/bin/bash 
set -eo pipefail

test_data="$1"
cp "${test_data}" ./data.txt

crabz -f bgzf -l6 ./data.txt > ./data.txt.gz
hyperfine \
    --warmup 2 \
    --runs 3 \
    --parameter-list num_threads 4,8,16 \
    './target/release/crabz -d -p {num_threads}  -f bgzf ./data.txt.gz > ./data.out.txt' \
    'bgzip -d -c -@ {num_threads}  ./data.txt.gz > ./data.out.txt'


hyperfine \
    --warmup 2 \
    --runs 3 \
    --parameter-list num_threads 2,4,8,16,32 \
    --parameter-list comp_level 6 \
    './target/release/crabz -p {num_threads} -l {comp_level} -f gzip ./data.txt > ./data.out.txt.gz' \
    './target/release/crabz -p {num_threads} -l {comp_level} -f bgzf ./data.txt > ./data.out.txt.gz' \
    'bgzip -c -@ {num_threads} -l {comp_level} ./data.txt > ./data.out.txt.gz' \
    'pigz -c -p {num_threads} -{comp_level} ./data.txt > ./data.out.txt.gz'
