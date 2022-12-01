#!/usr/bin/env bash

set -ue

SCRIPT_DIR=$( cd "$(dirname "$0")" ; pwd -P )
cd "${SCRIPT_DIR}"

DAY=${1:-$(date +%d)}
DAY_DIR="src/day${DAY}"

[ -d "${DAY_DIR}" ] && echo "Directory ${DAY_DIR} already exists" && exit 1

cp -r "src/day00" "${DAY_DIR}"
mv "${DAY_DIR}/day-00.txt" "${DAY_DIR}/day-${DAY}.txt"
mv "${DAY_DIR}/day-00-sample.txt" "${DAY_DIR}/day-${DAY}-sample.txt"

sed -i "s/00/${DAY}/g" "${DAY_DIR}/mod.rs"
