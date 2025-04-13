#!/bin/bash

function exists_in_list() {
    LIST=$1
    DELIMITER=$2
    VALUE=$3
    echo $LIST | tr "$DELIMITER" '\n' | grep -F -q -x "$VALUE"
}

input=$1

mkdir -p test-programs

if [[ $input = "testing-utils" ]]
then
    echo "building testing-utils"
    cd core/rust/testing-utils
    cargo build-bpf --bpf-out-dir ../../../test-programs/
    cd ../../../
else
    echo "Invalid program name: $input"
    echo "Available options: testing-utils"
    exit 1
fi


