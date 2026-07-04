#!/bin/sh
for mode in "debug" "release"; do
    echo "Mode is $mode"
    for feature in "" never infallible no-constructor; do
        echo "Feature is '$feature'"
        cargo_flags=()
        if [ $mode = "release" ]; then
            cargo_flags+=( --release )
        fi
        if [ ! -z "$feature" ]; then
            cargo_flags+=( --features "$feature" )
        fi

        echo cargo run "${cargo_flags[@]}" 2>/dev/null
        cargo run "${cargo_flags[@]}" 2>/dev/null
        echo "=== MyContainer strings ==="
        strings "target/$mode/never-test" | grep MyContainer
        echo
    done
    echo
    echo
done
