#!/usr/bin/env bash
# Usage: push-docker-image.sh tag

set -eu

TAG="$1"
image_id="$(docker image inspect --format='{{.Id}}' "imos/icfpc2020:${TAG}")"
image_id="${image_id##sha256:}"
image_id="${image_id:0:8}"
tag="${TAG}-$(date '+%Y%m%d')-${image_id}"
echo "Pushing ${tag}..." >&2
docker tag "imos/icfpc2020:${TAG}" "imos/icfpc2020:${tag}"
docker push "imos/icfpc2020:${tag}"
tmpfile=`mktemp`
echo -n "${tag}" > "${tmpfile}"
gsutil cp "${tmpfile}" "gs://icfpc-public-data/hash/docker-${TAG}"
rm "${tmpfile}"
docker push "imos/icfpc2020:${TAG}"
