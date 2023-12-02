#!/usr/bin/env bash

if [ $# -ne 1 ]; then
    echo "Require one argument, e.g.: ./new.sh day01"
    exit 1
fi

set -x
touch input/$1.txt
touch src/bin/$1.rs
