#!/bin/bash

# Create output directory
mkdir -p out

# Compile code and run executable
build_and_run() {
    echo "##### Day $1 #####"
    # Rust (https://www.rust-lang.org/tools/install)
    if [ -f day$1/main.rs ]; then
        rustc day$1/main.rs --edition 2018 -O -v -o out/day$1 && \
        cat day$1/input.txt | ./out/day$1
    fi
    # Kotlin (https://github.com/JetBrains/kotlin/releases/latest)
    if [ -f day$1/main.kt ]; then
        kotlinc -include-runtime -jvm-target 1.8 -d out/day$1.jar day$1/main.kt
        java -jar out/day$1.jar
    fi
    echo "##################"
    echo
}

# Days
#build_and_run 1
#build_and_run 2
#build_and_run 3
#build_and_run 4
#build_and_run 5
#build_and_run 6
#build_and_run 7
#build_and_run 8
build_and_run 9