
GO_WINNER="winner-consumer"
GO_LOSSER="losser-consumer"

DOCKERHUB_USERNAME="luiscastillo2001"
TAG_CONSUMERS="1.1"

sudo docker build -t $GO_WINNER ../golang/consumers/winners
sudo docker build -t $GO_LOSSER ../golang/consumers/lossers

docker tag $GO_WINNER "$DOCKERHUB_USERNAME/$GO_WINNER:$TAG_CONSUMERS"
docker tag $GO_LOSSER "$DOCKERHUB_USERNAME/$GO_LOSSER:$TAG_CONSUMERS"

docker push "$DOCKERHUB_USERNAME/$GO_WINNER:$TAG_CONSUMERS"
docker push "$DOCKERHUB_USERNAME/$GO_LOSSER:$TAG_CONSUMERS"