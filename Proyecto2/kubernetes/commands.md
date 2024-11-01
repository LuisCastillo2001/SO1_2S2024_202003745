
- Si

## Para exponer el cluster en nginx
kubectl create ns nginx-ingress
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx 
helm repo update 
helm install nginx-ingress ingress-nginx/ingress-nginx -n nginx-ingress
kubectl get services -n nginx-ingress
- Cambiar la IP en el ingress

## Comandos para aplicar el clúster

kubectl apply -f 'https://strimzi.io/install/latest?namespace=default' -n default
kubectl apply -f kafka.yaml -n default
kubectl wait kafka/my-cluster --for=condition=Ready --timeout=300s -n default 
kubectl apply -f kafka_topic.yaml -n default
kubectl apply -f atlserver.yaml -n default
kubectl apply -f boxingserver.yaml  -n default
kubectl apply -f swimmingser.yaml -n default
kubectl apply -f redis.yaml -n default
kubectl apply -f consumers.yaml  -n default
kubectl apply -f goclient.yaml  -n default
kubectl apply -f rustclient.yaml -n default
kubectl apply -f ingress.yaml   -n default

## Comandos para grafana
ACCOUNT=$(gcloud info --format='value(config.account)')
kubectl create clusterrolebinding owner-cluster-admin-binding \
    --clusterrole cluster-admin \
    --user $ACCOUNT

helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo update
helm install my-kube-prometheus-stack prometheus-community/kube-prometheus-stack
kubectl expose service my-kube-prometheus-stack-prometheus --type=NodePort --target-port=9090 --name=prometheus-node-port-service
kubectl expose service my-kube-prometheus-stack-grafana --type=NodePort --target-port=3000 --name=grafana-node-port-service

kubectl patch svc my-kube-prometheus-stack-grafana --namespace default -p '{"spec": {"type": "LoadBalancer"}}'
password - prom-operator

-- borrar
kubectl delete -f kafka_topic.yaml
kubectl exec redis-6b5bcbb6b6-9xp8k -- redis-cli FLUSHALL

35.227.76.159


## Para crear el cluster

gcloud container clusters create proyecto2 --num-nodes=4  --tags=allin,allout --machine-type=e2-medium --no-enable-network-policy --disk-size=25GB --disk-type pd-standard

gcloud container clusters create proyecto2 --num-nodes=4  --tags=allin,allout --machine-type=e2-medium --no-enable-network-policy --disk-size=25GB --disk-type pd-standard

region: us-east1
zone: us-east1-c
kubectl get pods redis

kubectl patch svc my-kube-prometheus-stack-grafana  --namespace default -p '{"spec": {"type": "LoadBalancer"}}




