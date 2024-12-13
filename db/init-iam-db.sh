#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username="$POSTGRES_USER" --dbname="$POSTGRES_DB" <<-EOSQL
  CREATE USER iam WITH ENCRYPTED PASSWORD 'iam';
  CREATE DATABASE iam WITH OWNER iam;
  GRANT ALL PRIVILEGES ON DATABASE iam TO iam;
EOSQL