[private]
default:
  @just --list --unsorted

setup:
  #!/bin/bash
  set -e
  if k3d cluster ls -o json | jq -r '.[] | select(.name == "sandcastle") | .name' | grep -q "sandcastle"; then
    echo "Sandcastle cluster already exists"
  else
    k3d cluster create -c config/dev/k3d/cluster.yaml
    kubectl wait --for='jsonpath={.status.conditions[?(@.type=="Failed")].status}=False' -n kube-system helmchart argo-cd
  fi

cleanup:
  k3d cluster delete -c config/dev/k3d/cluster.yaml
