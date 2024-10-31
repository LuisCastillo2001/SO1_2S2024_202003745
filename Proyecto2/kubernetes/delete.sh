
kubectl delete -f 'https://strimzi.io/install/latest?namespace=default' -n default
kubectl delete -f kafka.yaml
kubectl delete -f kafka_topic.yaml
kubectl delete -f atlserver.yaml
kubectl delete -f boxingserver.yaml
kubectl delete -f swimmingser.yaml
kubectl delete -f consumer.yaml
kubectl delete -f goclient.yaml
kubectl delete -f rustclient.yaml
kubectl delete -f ingress.yaml
