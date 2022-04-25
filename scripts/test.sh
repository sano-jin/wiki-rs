#!/bin/bash

show_help() {
    cat <<EOM
availale commands are ...
- get
- post
- delete
- help
EOM
}

case "$1" in
"get") curl -kX GET https://127.0.0.1:8443/"$2" ;;
"get_pages") curl -kX GET "https://127.0.0.1:8443/pages?path=$2" ;;
"get_edit") curl -kX GET "https://localhost:8443/edit?path=$2" ;;
"post") curl -H "content-type: application/json" -kX POST -d \
    "{\"path\": \"$2\", \"body\": \"$3\"}" \
    https://localhost:8443/edit ;;
"post2") curl -H "content-type: application/json" -kX POST -d "{\"name\": \"$2\", \"password\": \"$3\"}" https://localhost:8443/create_user ;;
"delete") curl -kX DELETE "https://localhost:8443/edit?path=$2" ;;
"help") show_help ;;
*)
    echo "Error: unknown command '$1'"
    show_help
    ;;
esac
