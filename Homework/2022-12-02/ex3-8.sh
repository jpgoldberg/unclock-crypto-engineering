#!/bin/bash

VERBOSE=false

MKTEMP=/usr/bin/mktemp
OPENSSL=/usr/bin/openssl
XXD=/usr/bin/xxd

CT38_TEMP="$(mktemp )" || { exit 1; }
PT38_TEMP="$(mktemp )" || { exit 1; }
CT39_TEMP="$(mktemp )" || { exit 1; }
PT39_TEMP="$(mktemp )" || { exit 1; }

trap cleanup 1 2 3 15 

cleanup() {
    rm -rf "$CT38_TEMP" "$CT39_TEMP" "$PT38_TEMP" "$PT39_TEMP"
}

verbose () {
    if [ $VERBOSE = true ]
    then
        echo "$@" 1>&2
    fi
}

# get ciphertext hex into a variable
ct=$(echo -n "53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07" | tr -d  ' ') 
verbose "ct:" $ct


key=$(echo -n "80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01" | tr -d ' ')
verbose "key:" $key

echo -n $ct | $XXD -r -p > "$CT38_TEMP"
verbose "ct.bin:" $($XXD -p < "$CT38_TEMP" )

$OPENSSL enc -d -aes-256-ecb -nopad -in "$CT38_TEMP" -K $key > "$PT38_TEMP"

echo "Ex 3.8 Plaintext:" $(xxd -p < "$PT38_TEMP" )

# Now onto 3.9 (but not the PGP stuff)
pt39=$(echo -n "29 6C 93 FD F4 99 AA EB 41 94 BA BC 2E 63 56 1D" | tr -d ' ')

echo -n $pt39 | $XXD -r -p > "$PT39_TEMP"

$OPENSSL enc -aes-256-ecb -nopad -in "$PT39_TEMP" -K "$key" > "$CT39_TEMP"

echo "Ex 3.9 ciphertext:" $($XXD -p < "$CT39_TEMP" )

cleanup

exit 0

