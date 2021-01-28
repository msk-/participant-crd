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
Deploy to a pod in the cluster, or run locally.

