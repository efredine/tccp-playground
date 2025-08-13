source .env
sysbench ./tpcc.lua \
  --pgsql-host=127.0.0.1 \
  --pgsql-port=5432 \
  --pgsql-password=${POSTGRES_PASSWORD} \
  --pgsql-user=${POSTGRES_USER} \
  --pgsql-db=${POSTGRES_DB} \
  --scale=10 \
  --db-driver=pgsql \
  prepare
