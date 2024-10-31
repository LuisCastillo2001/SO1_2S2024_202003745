#!/bin/bash

GO_SERVER_SWIMMING="swimming-server"
GO_SERVER_BOXING="boxing-server"
GO_SERVER_ATHLESTIM="atl-server"

DOCKERHUB_USERNAME="luiscastillo2001"
TAG_SERVERS="1.8"

# Build the Docker images for servers
sudo docker build -t $GO_SERVER_SWIMMING ../golang/servers/swimming_server
sudo docker build -t $GO_SERVER_BOXING ../golang/servers/boxing_server
sudo docker build -t $GO_SERVER_ATHLESTIM ../golang/servers/atl_server

# Tag the Docker images
docker tag $GO_SERVER_SWIMMING "$DOCKERHUB_USERNAME/$GO_SERVER_SWIMMING:$TAG_SERVERS"
docker tag $GO_SERVER_BOXING "$DOCKERHUB_USERNAME/$GO_SERVER_BOXING:$TAG_SERVERS"
docker tag $GO_SERVER_ATHLESTIM "$DOCKERHUB_USERNAME/$GO_SERVER_ATHLESTIM:$TAG_SERVERS"

# Push the Docker images to DockerHub
docker push "$DOCKERHUB_USERNAME/$GO_SERVER_SWIMMING:$TAG_SERVERS"
docker push "$DOCKERHUB_USERNAME/$GO_SERVER_BOXING:$TAG_SERVERS"
docker push "$DOCKERHUB_USERNAME/$GO_SERVER_ATHLESTIM:$TAG_SERVERS"

echo "Server Docker images pushed successfully."
