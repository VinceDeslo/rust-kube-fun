### Rust Kube Fun

Just a small repository to mess around with [kube-rs](https://github.com/kube-rs/kube/tree/main).

### Local testing
Spinning up a small pod locally to play around with:
```
minikube start --kubernetes-version=v1.26.0
kubectl create deployment web --image=gcr.io/google-samples/hello-app:1.0
```
