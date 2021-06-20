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

# service
kubectl apply -f ./adoption.yaml
kubectl apply -f ./animal-center.yaml