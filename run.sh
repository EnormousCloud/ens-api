#!/usr/bin/env bash
set -xe

[[ "$1" == "build" ]] && {
    shift
    docker build -t ens-rest-server .
}

[[ "$1" == "publish" ]] && {
    shift
    export SSH_HOST="root@enormous.cloud"
    docker save ens-rest-server | bzip2 | ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null  $SSH_HOST 'bunzip2 | docker load'
    ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null $SSH_HOST 'cd /opt/ens-rest-server; docker rm -f ens-rest-server; docker-compose up -d'
}

