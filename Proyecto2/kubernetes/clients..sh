#!/bin/bash

GO_CLIENT_IMAGE="golang-client-grpc"
RUST_CLIENT_IMAGE="rust-client-grpc"

DOCKERHUB_USERNAME="luiscastillo2001"
TAG_CLIENTS="6.6"

# Build the Docker images for clients
sudo docker build -t $GO_CLIENT_IMAGE ../golang/client
sudo docker build -t $RUST_CLIENT_IMAGE ../rust/grpc-client

# Tag the Docker images
docker tag $GO_CLIENT_IMAGE "$DOCKERHUB_USERNAME/$GO_CLIENT_IMAGE:$TAG_CLIENTS"
docker tag $RUST_CLIENT_IMAGE "$DOCKERHUB_USERNAME/$RUST_CLIENT_IMAGE:$TAG_CLIENTS"

# Push the Docker images to DockerHub
docker push "$DOCKERHUB_USERNAME/$GO_CLIENT_IMAGE:$TAG_CLIENTS"
docker push "$DOCKERHUB_USERNAME/$RUST_CLIENT_IMAGE:$TAG_CLIENTS"

echo "Client Docker images pushed successfully."
