#!/bin/sh

for i in "matmult_int" "nbody" "st" "aha_mont64" "crc32" "minver" "cubic" "nettle_aes"
do
    cargo run --release $i
done