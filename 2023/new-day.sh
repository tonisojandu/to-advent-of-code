#!/usr/bin/env bash

set -eu

## current day
DAY=$(date +%d)
NEW_FILE="src/day${DAY}.rs"

cp src/day00.rs src/day${DAY}.rs

sed -i 'replacebak' "s/00/${DAY}/g" ${NEW_FILE}
find . | grep replacebak | xargs rm
