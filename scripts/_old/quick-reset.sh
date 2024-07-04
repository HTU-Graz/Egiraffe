#!/usr/bin/env bash
# -*- coding: utf-8 -*-

EGNG_CONTAINER_IMAGE=postgres:16.1-bookworm
EGNG_CONTAINER_NAME=egiraffe_postgres
EGNG_DATABASE_NAME=egiraffe
EGNG_DATABASE_PASSWORD=test

EGNG_CONTAINER_IP=$(sudo docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "egiraffe_postgres")

EGNG_SCRIPT_DIR="$0:a:h"

egng-psql() {
    PGPASSWORD="$EGNG_DATABASE_PASSWORD" psql \
        -h "$EGNG_CONTAINER_IP" -U postgres \
        "$@"
}

echo ">> resetting database"
egng-psql \
    -c "DROP DATABASE IF EXISTS $EGNG_DATABASE_NAME;" \
    -c "CREATE DATABASE $EGNG_DATABASE_NAME;"
egng-psql \
    -d "$EGNG_DATABASE_NAME" \
    -f "$EGNG_SCRIPT_DIR/../../design/database/yeet.sql" \
    -f "$EGNG_SCRIPT_DIR/../../design/database/egiraffe-schema-generated.sql"
