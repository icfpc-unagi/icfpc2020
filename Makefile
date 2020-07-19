.PHONY: usage
usage:
	-echo 'Usage: make (docker|deploy-dashboard)'

.PHONY: test
test:
	cargo vendor
	cargo build
	cargo test
	./target/debug/galaxy < ./data/galaxy.txt | \
		diff - ./data/galaxy_expected.txt

.PHONY: build
build:
	cargo build

.PHONY: format
format:
	cargo fmt

.PHONY: submission
submission:
	@rm -rf build/submission
	bash script/build-submission.sh

.PHONY: submission-test
submission-test: submission
	cd build/submission && bash build.sh && bash run.sh http://imoz.jp test

.PHONY: performance
performance:
	bash test/performance_test.sh

.PHONY: deploy-dashboard
deploy-dashboard:
	cd go/cmd/dashboard && unagi --bare make deploy

.PHONY: bootstrap
bootstrap:
	cd docker && make bootstrap

.PHONY: unagi
unagi:
	cd docker && make docker

.PHONY: docker
docker:
	docker build -t imos/icfpc2020:submission .

.PHONY: launcher
launcher: docker/launcher

.PHONY: upload
upload: push-docker

.PHONY: upload-launcher
upload-launcher: docker/upload-launcher

.PHONY: upload-installer
upload-installer: docker/upload-installer

.PHONY: push-docker
push-docker: docker/push-docker-latest

.PHONY: docker/%
docker/%: unagi
	docker run -v $(shell pwd):/work -w /work \
		-v /var/run/docker.sock:/var/run/docker.sock \
		imos/icfpc2020 make "orig@$*"

################################################################################
# Targets run inside unagi image.
################################################################################

.PHONY: orig@launcher
orig@launcher:
	cd go/cmd/launcher && make -j 6
	cp script/launcher.sh build/launcher
	chmod +x build/launcher

.PHONY: orig@upload-launcher
orig@upload-launcher:
	cd go/cmd/launcher && make -j 6 upload

.PHONY: orig@upload-installer
orig@upload-installer:
	gsutil cp script/install-launcher.sh gs://icfpc-public-data/install.sh

.PHONY: orig@orig-upload
orig@upload: orig@upload-launcher orig@upload-installer

.PHONY: orig@push-docker-%
orig@push-docker-%:
	bash script/push-docker-image.sh "$*"
