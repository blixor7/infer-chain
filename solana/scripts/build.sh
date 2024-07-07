#!/bin/bash

# get program name from 1st positional argument
PROGRAM_NAME=$1

# check if program name is empty
if [ -z "${PROGRAM_NAME}" ]; then
	echo "error: missing program name"
	echo "usage: ./verifiable-build.sh <PROGRAM_NAME>"
	exit 1
fi

# get repo root for reference point
REPO_ROOT=$(git rev-parse --show-toplevel)/solana

pushd $REPO_ROOT/programs/$PROGRAM_NAME

cargo build-sbf

popd
