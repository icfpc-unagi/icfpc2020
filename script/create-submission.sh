#!/usr/bin/env bash

set -eux
shopt -s dotglob

cd "$(dirname "${BASH_SOURCE}")/.."

if [ -d build/submission ]; then
    rm -rf build/submission
fi
git clone -b submission --single-branch --depth=1 \
    git@github.com:imos/icfpc2020-submission.git build/submission
DATETIME="$(TZ=Asia/Tokyo date +%Y%m%d-%H%M%S)"
COMMIT_ID="$(git rev-parse --short HEAD)"
COMMIT_MSG="$(git log -n 1)"

git checkout -b "candidates/${DATETIME}-${COMMIT_ID}"
pushd build/submission
for f in *; do
    if [ "${f}" == '.git' ]; then
        continue
    fi
    rm -rf "${f}"
done
popd

cp -a app build/submission/app
cp -a Cargo.toml build/submission/
cp -a submission/* build/submission/

pushd build/submission
cargo vendor
popd

pushd build/submission
git add -A
git commit -am "${DATETIME}-${COMMIT_ID}"$'\n'"${COMMIT_MSG}"
git push --set-upstream origin "HEAD:candidates/${DATETIME}-${COMMIT_ID}"
popd
