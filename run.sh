#!/bin/bash

cargo run | while read OUTPUT; do notify-send "$OUTPUT"; done
