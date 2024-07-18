#!/bin/bash
INPUT_FILE=script.cbor
OUTPUT_FILE=script.bin

dd status=none if=$INPUT_FILE bs=1 skip=6 conv=notrunc | xxd -r -p > $OUTPUT_FILE
uplc convert --if flat -i $OUTPUT_FILE
