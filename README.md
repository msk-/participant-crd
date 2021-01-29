### TODO
- stricter compilation possible?
- replace u128 as the net_debit_cap type with num::BigRational (probably)
- GH actions

### Building
```sh
cargo build
```
#### Docker
```sh
docker build .
```
#### Nix
```sh
nix-shell rust.nix
cargo build
```

### Running
#### Local
```sh
cargo run
```

Nix:
```sh
nix-shell --command 'cargo run' rust.nix
```

#### In-cluster
1. Push to a branch $branch in GH, wait for the build action to complete
2. Create or update the deployment (this uses the current git branch)
```sh
yq "(.spec.template.spec.containers[] | select(.name == \"participant-controller\") | .image) |= \"ghcr.io/partiallyordered/participant-crd:sha-$(git rev-parse --short=7 @)\"" deploy.yaml | kubectl apply -f -
```
