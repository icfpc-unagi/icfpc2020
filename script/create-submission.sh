#!/usr/bin/env bash

set -eux
shopt -s dotglob

cd "$(dirname "${BASH_SOURCE}")/.."

if [ -d build/submission ]; then
    rm -rf build/submission
fi
if [ "${UNAGI_GITHUB_TOKEN:-}" != '' ]; then
    REPOSITORY_URI="https://$UNAGI_GITHUB_TOKEN@github.com/imos/icfpc2020-submission.git"
else
    REPOSITORY_URI=git@github.com:imos/icfpc2020-submission.git
fi
git clone -b submission --single-branch --depth=1 "${REPOSITORY_URI}" build/submission

DATETIME="$(TZ=Asia/Tokyo date +%Y%m%d-%H%M%S)"
COMMIT_ID="$(git rev-parse --short HEAD)"
COMMIT_MSG="$(git log -n 1)"

bash script/build-submission.sh

pushd build/submission
git add -A
git commit --allow-empty -am "${DATETIME}-${COMMIT_ID} #release"$'\n'"${COMMIT_MSG}"
git push --set-upstream origin "submission"
popd
