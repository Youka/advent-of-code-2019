#!/bin/bash

# Create output directory
mkdir -p out

# Compile code and run executable
build_and_run() {
    echo "##### Day $1 #####"
    rustc day$1/main.rs --edition 2018 -O -v -o out/day$1 && \
    cat day$1/input.txt | ./out/day$1
    echo "##################"
    echo
}

# Days
#build_and_run 1
#build_and_run 2
#build_and_run 3
#build_and_run 4
build_and_run 5