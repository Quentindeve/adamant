#!/bin/bash
cargo build
./meta/x86_64/build_limine.sh
bochs -q -f meta/x86_64/.bochsrc