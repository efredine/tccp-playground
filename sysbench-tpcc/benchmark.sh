source .env
./tpcc.lua \
  --pgsql-host=127.0.0.1 \
  --pgsql-port=5432 \
  --pgsql-password=${POSTGRES_PASSWORD} \
  --pgsql-user=${POSTGRES_USER} \
  --pgsql-db=${POSTGRES_DB} \
  --tables=1 \
  --scale=10 \
  --time=300 \
  --threads=8 \
  --report-interval=1 \
  --histogram=off \
  --percentile=99 \
  --db-driver=pgsql \
  run
