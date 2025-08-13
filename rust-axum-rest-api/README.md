# Rust Axum REST API

## Create database

```shell
docker compose up
```

## Prepare Database
As per instructions in `sysbench-tpcc`.

```shell
cd ../sysbench-tpcc
bash prepare.sh
```

## Run the API on localhost

```shell
cargo run
```

## Delete Database and Volume

```shell
docker compose down -v
```
