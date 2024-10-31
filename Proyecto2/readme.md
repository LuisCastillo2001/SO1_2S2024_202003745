## Proyecto 2 SO1

gcloud container clusters create proyecto2 --num-nodes=4  --tags=allin,allout --machine-type=e2-medium --no-enable-network-policy --disk-size=25GB --disk-type pd-standard

gcloud container clusters create proyecto2 --num-nodes=4  --tags=allin,allout --machine-type=e2-medium --no-enable-network-policy --disk-size=25GB --disk-type pd-standard

region: us-east1
zone: us-east1-c
kubectl get pods redis

kubectl patch svc my-kube-prometheus-stack-grafana  --namespace default -p '{"spec": {"type": "LoadBalancer"}}

hgetall