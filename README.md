# kistro - kubernetes distribution

This project is inspected by kURL.
We tended to build kURL compatible addOn/packages with Rust controller.

## Relation

```
 cluster
    -----------------------------------
    | manifest
    |    ------> node 1 (master)
    |    ------> node 2 (storage/..)
    ------------------------------------
```


## Cli Plan

### kistro login

#### Developer

Push pkg / manifest temp to repo
 - store index in cloud
 - json file as init index

#### Admin (Normal User)

Apply manifest / values
 - store data in cloud

### kistro pkg

to build addOn/packages for kistro / kURL.

kistro pkg init
 - with github action
kistro pkg build (local build)
kistro pkg commit
kistro pkg release

### kistro manifest

to manange manifest for cluster

### kistro cluster

### kistro node

### kistro get

kistro get cluster == kistro cluster info
kistro get node == kistro node info

## Dashboard plan


## Reference
* [kURL](https://github.com/replicatedhq/kURL)
* [kubeadm](https://github.com/kubernetes/kubeadm)
* [kubespray](https://github.com/kubernetes-sigs/kubespray)
* [k3s](https://github.com/k3s-io/k3s)
* [kind](https://github.com/kubernetes-sigs/kind)
