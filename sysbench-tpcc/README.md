# sysbench-tpcc

TPCC-like workload for sysbench 1.0.x.
**Make sure you are using sysbench 1.0.14 or better!**

This is NOT an implementation of TPCC workload. It is "TPCC-like" and uses only queries and schemas from TPCC specification. It does not respect the required "keying time", and functions as a closed loop contention benchmark on a fixed data set, rather than an open loop benchmark that scales with the number of warehouses. It also does not respect multiple other TPCC specification requirements. Please do not use sysbench-tpcc to generate TPC-C results for comparing between vendors, or please attach a similar disclaimer as to the TPCC-like nature.

# install sysbench (on a Mac with Postgres)

```shell
brew install sysbench
```

# create database in docker
```shell
docker compose up
```

# prepare data and tables
```shell
bash prepare.sh
```

# Run benchmark

`
./tpcc.lua --mysql-socket=/tmp/mysql.sock --mysql-user=root --mysql-db=sbt --time=300 --threads=64 --report-interval=1 --tables=10 --scale=100 --db-driver=mysql run
`

# Cleanup 

`
./tpcc.lua --mysql-socket=/tmp/mysql.sock --mysql-user=root --mysql-db=sbt --time=300 --threads=64 --report-interval=1 --tables=10 --scale=100 --db-driver=mysql cleanup
`
