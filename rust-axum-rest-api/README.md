# Rust Axum REST API

## Create and Prepare Database
As per instructions in `sysbench-tpcc`.

```shell
cd ../sysbench-tpcc
docker compose up
bash prepare.sh
```

## Run the API on localhost

```shell
cargo  run --release
```

## Delete Database and Volume

```shell
cd ../sysbench-tpcc 
docker compose down -v
```
