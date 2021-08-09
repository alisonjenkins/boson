#!/bin/bash
DATETIME="$(date +%s)"
OUTPUT_FILE="/tmp/$DATETIME-steam-launch-envvars.txt"
env > "$OUTPUT_FILE"
