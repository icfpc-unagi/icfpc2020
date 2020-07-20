#!/usr/bin/env bash

set -eux

DATETIME="$(date "+%Y%m%d-%H%M%S")"
ATTACKER="${1:-c090cc8}"
DEFENDER="${2:-c090cc8}"
ATTACKER_PORT="${3:-8001}"
DEFENDER_PORT="${4:-8002}"

DIR="logs/${DATETIME}-${ATTACKER}-vs-${DEFENDER}"
mkdir -p "${DIR}"

docker pull "imos/icfpc2020:${ATTACKER}"
docker pull "imos/icfpc2020:${DEFENDER}"

docker run --rm -p "${ATTACKER_PORT}:8080" --name="proxy-${ATTACKER_PORT}" imos/icfpc2020:proxy > "${DIR}/attacker.txt" &
docker run --rm -p "${DEFENDER_PORT}:8080" --name="proxy-${DEFENDER_PORT}" imos/icfpc2020:proxy > "${DIR}/defender.txt" &

KEYS=($(docker run --rm imos/icfpc2020:get_room))

docker run --rm --net=host "imos/icfpc2020:${ATTACKER}" "http://localhost:${ATTACKER_PORT}/aliens/send" "${KEYS[0]}" >"${DIR}/attacker-stdout.txt" 2>"${DIR}/attacker-stderr.txt" &
ATTACKER_PID=$!

docker run --rm --net=host "imos/icfpc2020:${DEFENDER}" "http://localhost:${DEFENDER_PORT}/aliens/send" "${KEYS[1]}" >"${DIR}/defender-stdout.txt" 2>"${DIR}/defender-stderr.txt" &
DEFENDER_PID=$!

exit_code=1
if wait "${ATTACKER_PID}" && wait "${DEFENDER_PID}"; then
	exit_code=1
fi

docker kill "proxy-${ATTACKER_PORT}" &
docker kill "proxy-${DEFENDER_PORT}" &
wait || true

exit "${exit_code}"
