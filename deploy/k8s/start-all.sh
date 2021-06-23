#!/bin/bash

# namespace
kubectl apply -f ./namespace.yaml

# grafana-loki
helm repo add grafana https://grafana.github.io/helm-charts
helm repo update
helm upgrade --install loki grafana/loki-stack --namespace=anima --set grafana.enabled=true,prometheus.enabled=true,prometheus.alertmanager.persistentVolume.enabled=false,prometheus.server.persistentVolume.enabled=false

# db 
kubectl apply -f ./sqldata.yaml

# mq
kubectl apply -f ./rabbitmq.yaml

# adoption configmap 
# kubectl create configmap adoption-cm --from-literal=ASPNETCORE_ENVIRONMENT=Product --from-file=appsettings=./appsettings.json -n animal
kubectl apply -f ./adoption-cm.yaml

# service
kubectl apply -f ./adoption.yaml
kubectl apply -f ./animal-center.yaml