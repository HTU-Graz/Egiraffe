#!/bin/bash

EGNG_CONTAINER_IMAGE=postgres:16.1-bookworm
EGNG_CONTAINER_NAME=egiraffe_postgres
EGNG_DATABASE_NAME=egiraffe
EGNG_DATABASE_PASSWORD=test
EGNG_DATABASE_WAIT=2

# --- get script dir ---

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    echo ">> error: please run this script as 'source scripts/setup-docker-db.sh'"
    exit 1
fi

EGNG_SCRIPT_DIR=$(realpath "$(dirname "${BASH_SOURCE[0]}")")

# --- check if docker commands need sudo ---

echo ">> checking if docker needs sudo"
if docker ps &>/dev/null; then
    DOCKER_SUDO=
else
    DOCKER_SUDO=sudo
fi

docker() {
    $DOCKER_SUDO command docker "$@"
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

egng-has-container() {
    [[ -n $(docker ps -a -q -f name="$EGNG_CONTAINER_NAME") ]]
}

egng-is-running() {
    egng-has-container && [[ $(docker container inspect -f '{{.State.Running}}' "$EGNG_CONTAINER_NAME") == "true" ]]
}

egng-get-container-ip() {
    EGNG_CONTAINER_IP=$(docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$EGNG_CONTAINER_NAME")
}

egng-set-database-url() {
    egng-get-container-ip
    export DATABASE_URL="postgres://postgres:$EGNG_DATABASE_PASSWORD@$EGNG_CONTAINER_IP/$EGNG_DATABASE_NAME"
}

egng-create-container() {
    if not egng-has-container; then
        echo ">> creating container"
        docker create -e POSTGRES_PASSWORD=test --name="$EGNG_CONTAINER_NAME" "$EGNG_CONTAINER_IMAGE"
    fi
}

egng-start-container() {
    if not egng-has-container; then
        egng-create-container
    fi

    if not egng-is-running; then
        echo ">> starting container"
        docker start "$EGNG_CONTAINER_NAME"
        egng-set-database-url
    fi
}

egng-stop-container() {
    if egng-is-running; then
        echo ">> stopping container"
        docker stop "$EGNG_CONTAINER_NAME"
    fi
}

egng-remove-container() {
    if egng-is-running; then
        egng-stop-container
    fi

    if egng-has-container; then
        echo ">> removing container"
        docker rm "$EGNG_CONTAINER_NAME"
    fi
}

egng-reset-db() {
    if not egng-is-running; then
        egng-start-container

        echo ">> waiting $EGNG_DATABASE_WAIT secs for database to come up"
        sleep "$EGNG_DATABASE_WAIT"
    fi

    echo ">> resetting database"
    PGPASSWORD="$EGNG_DATABASE_PASSWORD" psql \
        -h "$EGNG_CONTAINER_IP" -U postgres \
        -c "DROP DATABASE IF EXISTS $EGNG_DATABASE_NAME;" \
        -c "CREATE DATABASE $EGNG_DATABASE_NAME;"
    PGPASSWORD="$EGNG_DATABASE_PASSWORD" psql \
        -h "$EGNG_CONTAINER_IP" -U postgres -d "$EGNG_DATABASE_NAME" \
        -f "$EGNG_SCRIPT_DIR/../design/database/yeet.sql" \
        -f "$EGNG_SCRIPT_DIR/../design/database/egiraffe-schema-generated.sql"
}

if egng-has-container; then
    egng-start-container
else
    egng-reset-db
fi
