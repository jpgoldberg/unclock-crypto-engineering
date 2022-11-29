#!/bin/bash

VERBOSE=true

verbose () {
    if [ $VERBOSE = true ]
    then
        echo "$@"
    fi
}

# get ciphertext hex into a variable
ct=$(echo -n "53 9B 33 3B 39 70 6D 14 90 28 CF E1 D9 D4 A4 07" | tr -d  ' ') 
verbose "ct:" $ct


key=$(echo -n "80 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01" | tr -d ' ')
verbose "key:" $key

echo -n $ct | xxd -r -p > ct.bin
verbose "ct.bin:" $(xxd -p < ct.bin)

openssl enc -d -aes-256-ecb -nopad -in ct.bin -K $key > pt.bin

echo "Plaintext:" $(xxd -p < pt.bin)
