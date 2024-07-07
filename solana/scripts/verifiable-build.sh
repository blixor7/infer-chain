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

pushd $REPO_ROOT

# configuration
ROOTDIR=programs/$PROGRAM_NAME
RUST_IMAGE_TAG=1.79
SOLANA_CLI=v1.18.16
# separate folder form target/ to prevent Permission Denied error
HOST_FOLDER_ABSOLUTE_PATH=$REPO_ROOT/verified/$PROGRAM_NAME
CONTAINER_FOLDER_ABSOLUTE_PATH=/programs/target
IMAGE_NAME=solana-native-typescript

# display configuration
echo "Program: $PROGRAM_NAME"
echo "Rust: $RUST_IMAGE_TAG"
echo "Solana: $SOLANA_CLI"

# build the image and name it "app"
# prevent caching (start build from 0)
echo "Building image..."
docker build --build-arg "ROOTDIR=$ROOTDIR" --build-arg "RUST_IMAGE_TAG=$RUST_IMAGE_TAG" --build-arg "SOLANA_CLI=$SOLANA_CLI" -t $IMAGE_NAME $REPO_ROOT
echo "Sucessfully built image..."

# run image and build the files
echo "Building verifiable program..."
docker run --rm --name "build-$IMAGE_NAME" -v $HOST_FOLDER_ABSOLUTE_PATH:$CONTAINER_FOLDER_ABSOLUTE_PATH $IMAGE_NAME
echo "Successfully built verifiable program..."

# delete the image named {$image_name}
echo "Deleting app image..."
docker image rm $IMAGE_NAME
echo "Successfully deleted app image..."

popd
