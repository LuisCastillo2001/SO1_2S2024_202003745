# Proyecto 2 SO1

Este proyecto tiene como objetivo implementar una arquitectura basada en Google Kubernetes Engine (GKE) en Google Cloud Platform (GCP) para monitorear las Olimpiadas de la Universidad de San Carlos de Guatemala. Mediante el despliegue de contenedores y la gestión de tráfico a través de Kubernetes, el sistema será capaz de manejar grandes volúmenes de tráfico de participantes de las facultades de Ingeniería y Agronomía, quienes competirán en disciplinas como Natación, Boxeo y Atletismo. La visualización en tiempo real de los resultados será posible a través de Grafana, donde se mostrarán los datos analizados, como el conteo de medallas obtenidas por cada facultad.


## Herramientas principales del proyecto
- ### Kafka   
Apache Kafka es una plataforma distribuida para manejar flujos de datos en tiempo real. Diseñado para transmitir grandes cantidades de datos con baja latencia, Kafka permite enviar y recibir mensajes entre múltiples servicios (publicadores y consumidores). En este proyecto, los servidores de cada disciplina publicarán los resultados en Kafka, mientras que otros servicios consumirán estos mensajes para su almacenamiento y análisis.

- ### Grafana   
Grafana es una herramienta de visualización que permite crear paneles interactivos basados en datos. En este sistema, Grafana se usa para monitorear los datos de los participantes y su rendimiento en tiempo real, mostrando tableros que incluyen el conteo de ganadores y perdedores por facultad y disciplina. Estos dashboards ayudan a visualizar de manera clara y dinámica el desempeño en las olimpiadas.

- ### Prometheus  
 Prometheus es una herramienta de monitoreo de código abierto diseñada para recopilar métricas de aplicaciones y sistemas automáticamente. En Kubernetes, Prometheus permite supervisar el estado y rendimiento de los componentes de cada servicio desplegado, como el uso de CPU, memoria y latencia. Estas métricas se envían a Grafana, donde pueden visualizarse en paneles que brindan un resumen completo del estado y uso de recursos en el clúster.


## Kubernetes

Kubernetes es una plataforma de orquestación de contenedores de código abierto que se utiliza para gestionar aplicaciones basadas en contenedores. Originalmente desarrollado por Google, Kubernetes automatiza el despliegue, la gestión, el escalado y la recuperación de aplicaciones distribuidas, permitiendo que se ejecuten de manera eficiente y confiable en entornos de nube y on-premises.

### Características principales de Kubernetes

- Automatización de Despliegues: Kubernetes permite el despliegue de aplicaciones en contenedores, gestionando de manera automática el proceso de actualización y recuperación, lo que facilita mantener aplicaciones activas y en funcionamiento constante sin interrupciones.

- Escalado Automático: Kubernetes ajusta el número de contenedores (o "pods") en función de la demanda, asegurando que las aplicaciones cuenten con la capacidad necesaria en momentos de alta carga y reduciendo recursos cuando la carga es baja. Esto se conoce como Horizontal Pod Autoscaling (HPA), una funcionalidad clave en este proyecto.

- Balanceo de Carga: Distribuye el tráfico de red entre los contenedores que componen una aplicación, manteniendo el rendimiento y garantizando que ningún recurso se sobrecargue. Esto asegura que el tráfico generado por herramientas como Locust pueda manejarse de manera eficiente en el clúster.

- Recuperación Automática: Si un contenedor falla o no responde, Kubernetes lo reinicia automáticamente para mantener la aplicación operativa. Además, si un nodo completo en el clúster falla, Kubernetes reasigna las cargas a otros nodos disponibles, incrementando la resiliencia del sistema.

- Gestión de Configuración y Secretos: Kubernetes maneja la configuración y los secretos (como credenciales y claves API) de manera segura y eficiente, proporcionando acceso a la configuración sin exponer información sensible en el código.

- Portabilidad y Escalabilidad: Kubernetes es compatible con múltiples entornos de nube (como Google Cloud Platform, AWS y Azure) y permite despliegues híbridos y multinube, lo cual facilita el traslado de aplicaciones entre diferentes infraestructuras sin necesidad de modificar el código.


## Deployments

### Clientes

```	yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: grpc-client-go

spec:
  selector:
    matchLabels:
      app: grpc-client-go
  template:
    metadata:
      labels:
        app: grpc-client-go
    spec:
      containers:
      - name: grpc-client-go
        image: luiscastillo2001/golang-client-grpc:6.8
        resources:
          requests:
            cpu: "100m"
            memory: "128Mi"
          limits:
            cpu: "250m"
            memory: "256Mi"
        ports:
        - containerPort: 8080
---
apiVersion: v1
kind: Service
metadata:
  name: go-client-service #DNS
spec:
  selector:
    app: grpc-client-go
  ports:
    - protocol: TCP
      port: 8080 # Entrada
      targetPort: 8080 # Salida
  type: ClusterIP

---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: grpc-client-go-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: grpc-client-go
  minReplicas: 1
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 50
```
Implementa un cliente gRPC desarrollado en Go, configurado para ejecutarse con el nombre grpc-client-go. El contenedor, basado en la imagen luiscastillo2001/golang-client-grpc:6.8, utiliza recursos mínimos de CPU y memoria (con límites establecidos) para mantener su rendimiento controlado. Se expone a través de un Service de tipo ClusterIP bajo el nombre go-client-service, el cual permite la comunicación interna en el clúster por el puerto 8080. Además, el despliegue incluye un Horizontal Pod Autoscaler (HPA) que ajusta la cantidad de réplicas según el uso de CPU, permitiendo escalar dinámicamente de 1 a 10 pods para mantener una utilización media de CPU cercana al 50%, asegurando eficiencia y resiliencia ante variaciones en la carga de trabajo.

### Servidores

``` yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: grpc-atlser-go
spec:
  selector:
    matchLabels:
      app: grpc-atlser-go
  template:
    metadata:
      labels:
        app: grpc-atlser-go
    spec:
      containers:
      - name: grpc-atlser-go
        image: luiscastillo2001/atl-server:1.8
        resources:
          requests:
            cpu: "100m"
            memory: "128Mi"
          limits:
            cpu: "250m"
            memory: "256Mi"
        ports:
        - containerPort: 50051
---
apiVersion: v1
kind: Service
metadata:
  name: athletics-server #DNS
spec:
  selector:
    app: grpc-atlser-go
  ports:
    - protocol: TCP
      port: 50051
      targetPort: 50051
  type: ClusterIP
---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: grpc-atlser-go-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: grpc-atlser-go
  minReplicas: 1
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 50
```
Configura un servidor gRPC en Go para la disciplina de Atletismo, nombrado grpc-atlser-go. Utiliza la imagen luiscastillo2001/atl-server:1.8 y está optimizado con recursos limitados de CPU y memoria, asegurando un uso eficiente en el clúster. El servidor escucha en el puerto 50051 y está expuesto mediante un Service de tipo ClusterIP llamado athletics-server, permitiendo comunicación interna en el clúster para las solicitudes gRPC. Además, el despliegue cuenta con un Horizontal Pod Autoscaler (HPA) que ajusta dinámicamente el número de réplicas entre 1 y 10 según la utilización de CPU, con un objetivo del 50% de utilización media, proporcionando escalabilidad automática para manejar cambios en la carga de trabajo.
Nota: Es lo mismo para los otros dos servidores pero cambian los nombres.

### Kafka
``` yaml
apiVersion: kafka.strimzi.io/v1beta2
kind: KafkaNodePool
metadata:
  name: dual-role
  labels:
    strimzi.io/cluster: my-cluster
spec:
  replicas: 1
  roles:
    - controller
    - broker
  storage:
    type: jbod
    volumes:
      - id: 0
        type: persistent-claim
        size: 25Gi
        deleteClaim: false
        kraftMetadata: shared
---

apiVersion: kafka.strimzi.io/v1beta2
kind: Kafka
metadata:
  name: my-cluster
  annotations:
    strimzi.io/node-pools: enabled
    strimzi.io/kraft: enabled
spec:
  kafka:
    version: 3.8.0
    metadataVersion: 3.8-IV0
    listeners:
      - name: plain
        port: 9092
        type: internal
        tls: false
      - name: tls
        port: 9093
        type: internal
        tls: true
    config:
      offsets.topic.replication.factor: 1
      transaction.state.log.replication.factor: 1
      transaction.state.log.min.isr: 1
      default.replication.factor: 1
      min.insync.replicas: 1
  entityOperator:
    topicOperator: {}
    userOperator: {}

```

Esta configuración establece un clúster Kafka usando Strimzi en Kubernetes, que incluye un KafkaNodePool de un nodo con roles duales como controlador y broker. El almacenamiento está configurado en modo JBOD, con un volumen persistente de 25Gi asignado al nodo. La opción deleteClaim: false asegura que el volumen se conserve al eliminar el nodo.

La sección de Kafka define el clúster my-cluster, que utiliza el protocolo KRaft (Kafka Raft) en lugar de ZooKeeper para coordinar los nodos, habilitando una arquitectura simplificada. El clúster incluye dos tipos de listeners internos, uno en el puerto 9092 sin TLS y otro en el puerto 9093 con TLS. Las configuraciones de replicación están minimizadas para este entorno, permitiendo sólo un nodo en sincronización. Además, el clúster implementa un Entity Operator, que facilita la administración de usuarios y tópicos en Kafka, gestionándolos automáticamente dentro del clúster.

``` yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: redis
spec:
  replicas: 1
  selector:
    matchLabels:
      app: redis
  template:
    metadata:
      labels:
        app: redis
    spec:
      containers:
      - name: redis
        image: redis:latest  # Usar la última versión de Redis
        ports:
        - containerPort: 6379
---
apiVersion: v1
kind: Service
metadata:
  name: redis
spec:
  type: ClusterIP  # Puede ser NodePort o LoadBalancer si necesitas exposición externa
  ports:
  - port: 6379
    targetPort: 6379
  selector:
    app: redis
```
Este despliegue configura un contenedor Redis en Kubernetes para almacenamiento en memoria, proporcionando una instancia accesible internamente dentro del clúster en el puerto 6379. El Deployment especifica un único pod Redis, usando la última versión de la imagen, lo que permite aprovechar mejoras y correcciones recientes de Redis.

El Service de tipo ClusterIP expone Redis internamente al clúster, asegurando que otros pods pueden acceder a Redis usando su nombre de servicio (redis) y el puerto 6379.

# Comandos utilizados para la realización del proyecto

## Para exponer el cluster en nginx
```
kubectl create ns nginx-ingress
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx 
helm repo update 
helm install nginx-ingress ingress-nginx/ingress-nginx -n nginx-ingress
kubectl get services -n nginx-ingress
- Cambiar la IP en el ingress
```

## Comandos para aplicar el clúster
```
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
```

```javascript
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

-- Limpiar base de datos

kubectl delete -f kafka_topic.yaml
kubectl exec redis-6b5bcbb6b6-9xp8k -- redis-cli FLUSHALL
```

## Para crear el cluster

```
gcloud container clusters create proyecto2 --num-nodes=4  --tags=allin,allout --machine-type=e2-medium --no-enable-network-policy --disk-size=25GB --disk-type pd-standard

region: us-east1
zone: us-east1-c
```







