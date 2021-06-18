#!/bin/bash

kubectl apply \
    -f ./namespace.yaml \
    -f ./sqldata.yaml \
    -f ./adoption.yaml