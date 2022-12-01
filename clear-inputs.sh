#!/usr/bin/env bash

set -ue

SCRIPT_DIR=$( cd "$(dirname "$0")" ; pwd -P )
cd "${SCRIPT_DIR}"

for f in $( ls 2021/app/src/main/resources/Day* ) ; do
	echo "" > $f
	git add $f
done


for f in $( find 2022/src/ | grep main.txt ) ; do
	echo "" > $f
	git add $f
done
