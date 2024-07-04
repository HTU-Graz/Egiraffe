#!/usr/bin/env bash
# -*- coding: utf-8 -*-
# compatible with bash and zsh

EGNG_CONTAINER_IMAGE=postgres:16.1-bookworm
EGNG_CONTAINER_NAME=egiraffe_postgres
EGNG_DATABASE_NAME=egiraffe
EGNG_DATABASE_PASSWORD=test

# --- get script dir ---

EGNG_SOURCED=1
if [ -n "$BASH" ]; then
    # bash
    set +o posix
    EGNG_SCRIPT_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")
    if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
        echo ">> warning: this script is intended to be sourced"
        echo ">> warning: please run this script as 'source scripts/setup-docker-db.sh"
        EGNG_SOURCED=0
    fi
elif [ -n "$ZSH_EVAL_CONTEXT" ]; then
    # zsh
    EGNG_SCRIPT_DIR="$0:a:h"
    if [[ "$ZSH_EVAL_CONTEXT" == "toplevel" ]]; then
        echo ">> warning: this script is intended to be sourced"
        echo ">> warning: please run this script as 'source scripts/setup-docker-db.sh"
        EGNG_SOURCED=0
    fi
else
    # other shell
    echo ">> error: this script is only compatible with bash and zsh"
    echo ">> note: this script is intended to be sourced"
    echo ">> note: please run this script as 'source scripts/setup-docker-db.sh"
    exit 1
fi

# --- check if docker commands need sudo ---

echo ">> checking if docker needs sudo"
if docker ps &>/dev/null; then
    DOCKER_NEEDS_SUDO=0
else
    DOCKER_NEEDS_SUDO=1
fi

egng-docker() {
    if [[ "$DOCKER_NEEDS_SUDO" -eq 1 ]]; then
        sudo docker "$@"
    else
        docker "$@"
    fi
}

# --- helper functions ---

not() {
    if "$@"; then
        return 1
    else
        return 0
    fi
}

# --- container management functions ---

egng-usage() {
    echo ">> this script provides the following commands:"
    echo "- egng-has-container ...... check if DB container already created"
    echo "- egng-is-running ......... check if DB container is already running"
    echo "- egng-create-container ... create DB container"
    echo "- egng-start-container .... start DB container"
    echo "- egng-stop-container ..... stop DB container"
    echo "- egng-remove-container ... remove DB container"
    echo "- egng-reset-db ........... reset DB in container to defaults"
}

egng-has-container() {
    [[ -n $(egng-docker ps -a -q -f name="$EGNG_CONTAINER_NAME") ]]
}

egng-is-running() {
    egng-has-container && [[ $(egng-docker container inspect -f '{{.State.Running}}' "$EGNG_CONTAINER_NAME") == "true" ]]
}

egng-get-container-ip() {
    EGNG_CONTAINER_IP=$(egng-docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$EGNG_CONTAINER_NAME")
}

egng-set-database-url() {
    egng-get-container-ip
    export DATABASE_URL="postgres://postgres:$EGNG_DATABASE_PASSWORD@$EGNG_CONTAINER_IP/$EGNG_DATABASE_NAME"
}

egng-create-container() {
    if not egng-has-container; then
        echo ">> creating container"
        egng-docker create -e POSTGRES_PASSWORD=test --name="$EGNG_CONTAINER_NAME" "$EGNG_CONTAINER_IMAGE"
    fi
}

egng-start-container() {
    egng-create-container

    if not egng-is-running; then
        echo ">> starting container"
        egng-docker start "$EGNG_CONTAINER_NAME"
    fi

    egng-set-database-url
}

egng-stop-container() {
    if egng-is-running; then
        echo ">> stopping container"
        egng-docker stop "$EGNG_CONTAINER_NAME"
    fi
}

egng-remove-container() {
    egng-stop-container

    if egng-has-container; then
        echo ">> removing container"
        egng-docker rm "$EGNG_CONTAINER_NAME"
    fi
}

egng-psql() {
    PGPASSWORD="$EGNG_DATABASE_PASSWORD" psql \
        -h "$EGNG_CONTAINER_IP" -U postgres \
        "$@"
}

egng-wait-db() {
    egng-start-container

    echo ">> waiting until database is up"
    secs=0
    until egng-psql -c "SELECT 1" &>/dev/null; do
        secs=$((secs + 1))
        echo -n "."
        sleep 1
    done

    if [[ $secs -gt 0 ]]; then
        echo
    fi

    echo ">> database up after $secs seconds"
}

egng-reset-db() {
    egng-wait-db

    echo ">> resetting database"
    egng-psql \
        -c "DROP DATABASE IF EXISTS $EGNG_DATABASE_NAME;" \
        -c "CREATE DATABASE $EGNG_DATABASE_NAME;"
    # egng-psql \
    #     -d "$EGNG_DATABASE_NAME" \
    #     -f "$EGNG_SCRIPT_DIR/../design/database/yeet.sql" \
    #     -f "$EGNG_SCRIPT_DIR/../design/database/egiraffe-schema-generated.sql"
}

if egng-has-container; then
    egng-start-container
else
    egng-reset-db
fi

if [[ $EGNG_SOURCED -eq 1 ]]; then
    egng-usage
fi
